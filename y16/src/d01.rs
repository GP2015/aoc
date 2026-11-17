use common::PartAnswer;
use core::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Add<Turn> for Direction {
    type Output = Self;

    fn add(self, rhs: Turn) -> Self::Output {
        match (self, rhs) {
            (Self::East, Turn::Left) | (Self::West, Turn::Right) => Self::North,
            (Self::South, Turn::Left) | (Self::North, Turn::Right) => Self::East,
            (Self::West, Turn::Left) | (Self::East, Turn::Right) => Self::South,
            (Self::North, Turn::Left) | (Self::South, Turn::Right) => Self::West,
        }
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    dir: Direction,
    steps: u16,
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut dir = Direction::North;

    let instrs = input
        .split(|&b| b == b' ')
        .map(|chunk| {
            let (&last_item, rest) = chunk.split_last().unwrap();
            if last_item == b',' { rest } else { chunk }
        })
        .map(|raw_instr| {
            let (&raw_dir, raw_steps) = raw_instr.split_first().unwrap();

            dir += match raw_dir {
                b'L' => Turn::Left,
                b'R' => Turn::Right,
                _ => unreachable!(),
            };

            Instruction {
                dir,
                steps: raw_steps
                    .iter()
                    .fold(0, |acc, digit| (acc * 10) + u16::from(digit - b'0')),
            }
        });

    let mut pos = [0, 0];

    for instr in instrs {
        match instr.dir {
            Direction::North => pos[1] += i32::from(instr.steps),
            Direction::East => pos[0] += i32::from(instr.steps),
            Direction::South => pos[1] -= i32::from(instr.steps),
            Direction::West => pos[0] -= i32::from(instr.steps),
        }
    }

    let end = pos[0].abs() + pos[1].abs();

    [end.into(), 0.into()]
}
