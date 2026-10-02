use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use alloc::{vec, vec::Vec};
use arrayvec::ArrayVec;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Copy, Debug)]
enum InstrStem {
    TurnOn,
    TurnOff,
    Toggle,
}

fn read_number(input_idx: &mut usize, input: &[u8]) -> u16 {
    #[allow(clippy::indexing_slicing)]
    let chunk: [u8; 3] = input[*input_idx..*input_idx + 3].try_into().unwrap();

    let mut num = u16::from(chunk[0] - b'0');

    for &c in &chunk[1..] {
        if !c.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += u16::from(c - b'0');

        *input_idx += 1;
    }

    *input_idx += 2;

    num
}

fn read_line(input_idx: &mut usize, input: &[u8]) {
    let (stem, inc) = if input[*input_idx + 1] == b'o' {
        (InstrStem::Toggle, 7)
    } else if input[*input_idx + 6] == b'n' {
        (InstrStem::TurnOn, 8)
    } else {
        (InstrStem::Toggle, 9)
    };

    *input_idx += inc;

    let x_from = read_number(input_idx, input);
    let y_from = read_number(input_idx, input);
}

struct Instruction {
    instr_type: InstrStem,
    node_from: [u16; 2],
    node_to: [u16; 2],
}

impl From<&[u8]> for Instruction {
    fn from(raw: &[u8]) -> Self {
        let words: Vec<_> = raw.split(|&c| c == b' ').collect();

        let node = |s: &[u8]| {
            s.split(|&c| c == b',')
                .map(|s| {
                    let mut num = 0;
                    for &byte in s {
                        num *= 10;
                        num += u16::from(byte - b'0');
                    }
                    num
                })
                .next_array()
                .unwrap()
        };

        Self {
            instr_type: if words.len() == 4 {
                InstrStem::Toggle
            } else if words[1].len() == 2 {
                InstrStem::TurnOn
            } else {
                InstrStem::TurnOff
            },

            node_from: node(words[words.len() - 3]),
            node_to: node(words[words.len() - 1]),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Light {
    dig: bool,
    ana: u32,
}

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let mut grid = vec![[Light::default(); 1000]; 1000].into_boxed_slice();

        for instr in input.split(|&c| c == b'\n').map(Instruction::from) {
            let range = |i| instr.node_from[i] as usize..instr.node_to[i] as usize + 1;

            grid[range(0)]
                .par_iter_mut()
                .flat_map(|row| row[range(1)].par_iter_mut())
                .for_each(|light| match instr.instr_type {
                    InstrStem::TurnOn => {
                        light.dig = true;
                        light.ana += 1;
                    }
                    InstrStem::TurnOff => {
                        light.dig = false;
                        light.ana = light.ana.saturating_sub(1);
                    }
                    InstrStem::Toggle => {
                        light.dig = !light.dig;
                        light.ana += 2;
                    }
                });
        }

        grid.iter()
            .flat_map(|row| row.iter())
            .fold([0, 0], |[dig_acc, ana_acc], light| {
                [dig_acc + u32::from(light.dig), ana_acc + light.ana]
            })
            .map(ArrayVec::from_int)
    }
}
