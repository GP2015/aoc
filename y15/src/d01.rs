use crate::PartAnswer;
use core::hint;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut iter = input.iter().map(|&b| if b == b'(' { 1 } else { -1 });

    let mut floor = 0;
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

    [floor.into(), pos.into()]
}
