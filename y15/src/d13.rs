use alloc::vec;
use common::FromUnsigned;
use common::PartAnswer;
use itertools::Itertools;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let num_lines = 1 + bytecount::count(input, b'\n');
    let num_people = (1 + 4 * num_lines).isqrt().div_ceil(2);

    let mut graph = vec![0; num_people * num_people].into_boxed_slice();

    let vals = input.split(|&c| c == b'\n').map(|line| {
        let [op_raw, num_raw] = line.split(|&c| c == b' ').skip(2).next_array().unwrap();
        let num = i32::from_unsigned(num_raw);
        if op_raw[0] == b'g' { num } else { -num }
    });

    let mut idx = 1;
    for val in vals {
        graph[idx] = val;

        let extra_inc = (idx + 1).is_multiple_of(num_people + 1);
        idx += if extra_inc { 2 } else { 1 };
    }

    let a = (1..num_people)
        .permutations(num_people - 1)
        .filter(|perm| perm[0] < *perm.last().unwrap())
        .map(|perm| {
            let perm_total = perm
                .array_windows::<2>()
                .map(|[p0, p1]| graph[p0 * num_people + p1] + graph[p1 * num_people + p0])
                .sum::<i32>();

            let edge_total = [
                perm[0],
                perm[0] * num_people,
                perm[num_people - 2],
                perm[num_people - 2] * num_people,
            ]
            .into_iter()
            .map(|idx| graph[idx])
            .sum::<i32>();

            perm_total + edge_total
        })
        .max()
        .unwrap()
        .into();

    let b = (0..num_people)
        .permutations(num_people)
        .filter(|perm| perm[0] < *perm.last().unwrap())
        .map(|perm| {
            perm.array_windows::<2>()
                .map(|[p0, p1]| graph[p0 * num_people + p1] + graph[p1 * num_people + p0])
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .into();

    [a, b]
}
