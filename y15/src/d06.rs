use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use alloc::vec;
use arrayvec::ArrayVec;
use core::ops::Range;

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

    *input_idx += 1;

    num
}

fn read_line(input_idx: &mut usize, input: &[u8]) -> Instruction {
    let (instr_type, inc) = if input[*input_idx + 1] == b'o' {
        (InstrStem::Toggle, 7)
    } else if input[*input_idx + 6] == b'n' {
        (InstrStem::TurnOn, 8)
    } else {
        (InstrStem::TurnOff, 9)
    };

    *input_idx += inc;

    let x_from = read_number(input_idx, input);
    *input_idx += 1;
    let y_from = read_number(input_idx, input);

    *input_idx += 9;

    let x_to = read_number(input_idx, input);
    *input_idx += 1;
    let y_to = read_number(input_idx, input);
    *input_idx += 1;

    Instruction {
        instr_type,
        x_range: x_from as usize..x_to as usize + 1,
        y_range: y_from as usize..y_to as usize + 1,
    }
}

struct Instruction {
    instr_type: InstrStem,
    x_range: Range<usize>,
    y_range: Range<usize>,
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

        let mut input_idx = 0;

        while input_idx < input.len() {
            let instr = read_line(&mut input_idx, input);

            grid[instr.x_range]
                .iter_mut()
                .flat_map(|row| row[instr.y_range.clone()].iter_mut())
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
