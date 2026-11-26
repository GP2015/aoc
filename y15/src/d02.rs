use common::{FromUnsigned, PartAnswer};
use itertools::Itertools;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut dims = line
                .split(|&c| c == b'x')
                .map(u32::from_unsigned)
                .next_array()
                .unwrap();

            if dims[0] > dims[1] {
                dims.swap(0, 1);
            }

            if dims[1] > dims[2] {
                dims.swap(1, 2);
            }

            dims
        })
        .fold([0, 0], |[a, b], [d0, d1, d2]| {
            [
                a + 3 * d0 * d1 + 2 * d2 * (d0 + d1),
                b + 2 * (d0 + d1) + (d0 * d1 * d2),
            ]
        })
        .map(Into::into)
}
