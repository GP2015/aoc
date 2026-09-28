use crate::{
    IsDay,
    common::{ArrayVecConvert, MAX_OUTPUT_LEN},
};
use arrayvec::ArrayVec;

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let mut a = 0;
        let mut b = 0;

        let mut c_idx = 0;

        while c_idx < input.len() {
            let mut dims = [0; 3];

            for dim in &mut dims {
                *dim = u32::from(input[c_idx] - b'0');
                c_idx += 1;

                if c_idx < input.len() && input[c_idx].is_ascii_digit() {
                    *dim *= 10;
                    *dim += u32::from(input[c_idx] - b'0');
                    c_idx += 1;
                }

                c_idx += 1;
            }

            if dims[0] > dims[1] {
                dims.swap(0, 1);
            }

            if dims[1] > dims[2] {
                dims.swap(1, 2);
            }

            a += 3 * dims[0] * dims[1] + 2 * dims[2] * (dims[0] + dims[1]);
            b += 2 * (dims[0] + dims[1]) + (dims[0] * dims[1] * dims[2]);
        }

        [a, b].map(ArrayVec::from_int)
    }
}
