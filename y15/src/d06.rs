use alloc::{boxed::Box, vec};
use common::PartAnswer;
use core::fmt::Debug;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum InstrStem {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    stem: InstrStem,
    from_node: [u16; 2],
    to_node: [u16; 2],
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Grid<T>(Box<[T; 1_000_000]>);

impl<T> Grid<T>
where
    T: Debug + Default + Clone + Copy,
{
    pub fn new() -> Self {
        Self(
            vec![T::default(); 1_000_000]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        )
    }

    pub fn set(&mut self, from_node: [u16; 2], to_node: [u16; 2], effect: impl Fn(T) -> T) {
        for row in from_node[0] as usize..=to_node[0] as usize {
            for col in from_node[1] as usize..=to_node[1] as usize {
                let idx = row * 1000 + col;
                #[allow(clippy::indexing_slicing)]
                let light = &mut self[idx];
                *light = effect(*light);
            }
        }
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut digital_grid = Grid::<bool>::new();
    let mut analogue_grid = Grid::<u32>::new();

    let instrs = input.split(|&c| c == b'\n').map(|line| {
        let mut words = line.split(|&c| c == b' ');

        let stem = if words.next().unwrap()[1] == b'o' {
            InstrStem::Toggle
        } else if words.next().unwrap()[1] == b'n' {
            InstrStem::TurnOn
        } else {
            InstrStem::TurnOff
        };

        let node = |num: &[u8]| {
            num.split(|&c| c == b',')
                .map(|num| atoi::atoi(num).unwrap())
                .next_array::<2>()
                .unwrap()
        };

        let (from_raw, _, to_raw) = words.next_tuple().unwrap();

        Instruction {
            stem,
            from_node: node(from_raw),
            to_node: node(to_raw),
        }
    });

    for instr in instrs {
        macro_rules! set {
            ($grid:ident, $effect:expr) => {
                $grid.set(instr.from_node, instr.to_node, $effect)
            };
        }

        match instr.stem {
            InstrStem::TurnOn => set!(digital_grid, |_| true),
            InstrStem::TurnOff => set!(digital_grid, |_| false),
            InstrStem::Toggle => set!(digital_grid, |s| !s),
        }

        match instr.stem {
            InstrStem::TurnOn => set!(analogue_grid, |s| s + 1),
            InstrStem::TurnOff => set!(analogue_grid, |s| s.saturating_sub(1)),
            InstrStem::Toggle => set!(analogue_grid, |s| s + 2),
        }
    }

    [
        digital_grid.iter().filter(|&&light| light).count().into(),
        analogue_grid.iter().sum::<u32>().into(),
    ]
}
