use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Copy, Debug)]
enum InstructionStem {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    instr_type: InstructionStem,
    node_from: [u16; 2],
    node_to: [u16; 2],
}

impl From<&[u8]> for Instruction {
    fn from(raw: &[u8]) -> Self {
        let words: Vec<_> = raw.split(|&c| c == b' ').collect();

        let node = |s: &[u8]| {
            s.split(|&c| c == b',')
                .map(|s| atoi::atoi(s).unwrap())
                .next_array()
                .unwrap()
        };

        Self {
            instr_type: if words.len() == 4 {
                InstructionStem::Toggle
            } else if words[1].len() == 2 {
                InstructionStem::TurnOn
            } else {
                InstructionStem::TurnOff
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

pub fn run(input: &[u8]) -> [String; 2] {
    let mut grid = vec![[Light::default(); 1000]; 1000].into_boxed_slice();

    for instr in input.split(|&c| c == b'\n').map(Instruction::from) {
        let range = |i| instr.node_from[i] as usize..instr.node_to[i] as usize + 1;

        grid[range(0)]
            .par_iter_mut()
            .flat_map(|row| row[range(1)].par_iter_mut())
            .for_each(|light| match instr.instr_type {
                InstructionStem::TurnOn => {
                    light.dig = true;
                    light.ana += 1;
                }
                InstructionStem::TurnOff => {
                    light.dig = false;
                    light.ana = light.ana.saturating_sub(1);
                }
                InstructionStem::Toggle => {
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
        .map(|i| i.to_string())
}
