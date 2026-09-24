use crate::IsDay;
use std::io::{Read, StdinLock};

pub struct Day;

impl IsDay for Day {
    fn part_a(stdin_handle: StdinLock) {
        let floor = stdin_handle
            .bytes()
            .map(Result::unwrap)
            .fold(0, |acc, b| match b {
                b'(' => acc + 1,
                b')' => acc - 1,
                _ => unreachable!(),
            });

        println!("{floor}");
    }

    fn part_b(stdin_handle: StdinLock) {
        let mut floor = 0;

        for (pos, instr) in stdin_handle.bytes().map(Result::unwrap).enumerate() {
            match instr {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => unreachable!(),
            }

            if floor < 0 {
                println!("{}", pos + 1);
                return;
            }
        }
    }
}
