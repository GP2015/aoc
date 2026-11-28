use alloc::vec::Vec;
use common::{FromUnsigned, PartAnswer};
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct WireId(u16);

impl From<&[u8]> for WireId {
    fn from(word: &[u8]) -> Self {
        Self(if word.len() == 1 {
            u16::from(word[0] - b'a')
        } else {
            u16::from(word[0] - b'a' + 1) * 26 + u16::from(word[1] - b'a')
        })
    }
}

const A_ID: WireId = WireId(0);
const B_ID: WireId = WireId(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Param {
    Value(u16),
    Wire(WireId),
}

impl From<&[u8]> for Param {
    fn from(word: &[u8]) -> Self {
        if word[0].is_ascii_digit() {
            Self::Value(u16::from_unsigned(word))
        } else {
            Self::Wire(word.into())
        }
    }
}

impl Param {
    pub fn defined_in(self, wires: &FxHashMap<WireId, Option<u16>>) -> bool {
        match self {
            Self::Value(_) => true,
            Self::Wire(w) => wires[&w].is_some(),
        }
    }

    pub fn read(self, wires: &FxHashMap<WireId, Option<u16>>) -> u16 {
        match self {
            Self::Value(v) => v,
            Self::Wire(w) => wires[&w].unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Short(Param),
    Not(Param),
    Or(Param, Param),
    And(Param, Param),
    LShift(Param, Param),
    RShift(Param, Param),
}

impl From<&[&[u8]]> for Operation {
    fn from(words: &[&[u8]]) -> Self {
        let len = words.len();

        let words: &[&[u8]; 3] = words[..3].try_into().unwrap();

        match len {
            3 => Self::Short(words[0].into()),
            4 => Self::Not(words[1].into()),
            _ => {
                let op0 = words[0].into();
                let op1 = words[2].into();
                match words[1][0] {
                    b'O' => Self::Or(op0, op1),
                    b'A' => Self::And(op0, op1),
                    b'L' => Self::LShift(op0, op1),
                    b'R' => Self::RShift(op0, op1),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Operation {
    pub fn defined_in(&self, wires: &FxHashMap<WireId, Option<u16>>) -> bool {
        match self {
            Self::Short(p) | Self::Not(p) => p.defined_in(wires),
            Self::Or(p0, p1) | Self::And(p0, p1) | Self::LShift(p0, p1) | Self::RShift(p0, p1) => {
                p0.defined_in(wires) && p1.defined_in(wires)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    op: Operation,
    out: WireId,
}

impl From<&[&[u8]]> for Instruction {
    fn from(words: &[&[u8]]) -> Self {
        Self {
            op: Operation::from(words),
            out: (*words.last().unwrap()).into(),
        }
    }
}

fn get_wire_a(instrs: &[Instruction]) -> u16 {
    let mut wires = FxHashMap::from_iter(instrs.iter().map(|instr| (instr.out, None)));

    while wires[&A_ID].is_none() {
        for instr in instrs {
            if !instr.op.defined_in(&wires) {
                continue;
            }

            macro_rules! set {
                ($num:expr) => {
                    _ = wires.insert(instr.out, Some($num))
                };
            }

            match &instr.op {
                Operation::Short(p) => set!(p.read(&wires)),
                Operation::Not(p) => set!(!p.read(&wires)),
                Operation::Or(p0, p1) => set!(p0.read(&wires) | p1.read(&wires)),
                Operation::And(p0, p1) => set!(p0.read(&wires) & p1.read(&wires)),
                Operation::LShift(p0, p1) => set!(p0.read(&wires) << p1.read(&wires)),
                Operation::RShift(p0, p1) => set!(p0.read(&wires) >> p1.read(&wires)),
            }
        }
    }

    wires[&A_ID].unwrap()
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut instrs: Vec<_> = input
        .split(|&c| c == b'\n')
        .map(|line| {
            let words: Vec<_> = line.split(|&c| c == b' ').collect();
            Instruction::from(words.as_ref())
        })
        .collect();

    let a = get_wire_a(&instrs);

    for instr in &mut instrs {
        if instr.out == B_ID {
            instr.op = Operation::Short(Param::Value(a));
        }
    }

    [a.into(), get_wire_a(&instrs).into()]
}
