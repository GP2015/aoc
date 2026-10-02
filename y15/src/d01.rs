use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use arrayvec::ArrayVec;
use core::hint;

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let mut iter = input
            .iter()
            .map(|&b| hint::select_unpredictable(b == b'(', 1, -1));

        let mut floor: i32 = 0;
        let mut pos = 0;

        loop {
            #[allow(clippy::cast_sign_loss)]
            let step = floor as usize + 1;

            floor += iter.by_ref().take(step).sum::<i32>();
            pos += step;

            if floor < 0 {
                hint::cold_path();
                break;
            }
        }

        floor += iter.sum::<i32>();

        [ArrayVec::from_int(floor), ArrayVec::from_int(pos)]
    }
}
