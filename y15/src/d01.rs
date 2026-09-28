use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use arrayvec::ArrayVec;

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let mut iter = input.iter().map(|&b| match b {
            b'(' => 1,
            _ => -1, // b')'
        });

        let mut floor = 0;

        let pos = iter
            .position(|instr| {
                floor += instr;
                floor < 0
            })
            .unwrap()
            + 1;

        floor += iter.sum::<i32>();

        [ArrayVec::from_int(floor), ArrayVec::from_int(pos)]
    }
}
