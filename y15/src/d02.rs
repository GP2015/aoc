use common::PartAnswer;
use itertools::Itertools;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            let mut dims: [u32; 3] = line
                .split(|&c| c == b'x')
                .map(|num| atoi::atoi(num).unwrap())
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
        .fold([0, 0], |mut acc, dims| {
            acc[0] += 3 * dims[0] * dims[1] + 2 * dims[2] * (dims[0] + dims[1]);
            acc[1] += 2 * (dims[0] + dims[1]) + (dims[0] * dims[1] * dims[2]);
            acc
        })
        .map(Into::into)
}
