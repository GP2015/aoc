use alloc::{vec, vec::Vec};
use common::PartAnswer;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let num_lines = 1 + bytecount::count(input, b'\n');

    let num_cities = (1 + (1 + 8 * num_lines).isqrt()).div_ceil(2);

    let mut cities = Vec::with_capacity(num_cities);
    let mut dists = vec![0; num_cities * num_cities].into_boxed_slice();

    for line in input.split(|&c| c == b'\n') {
        let (l0, _, l1, _, raw_dist) = line.split(|&c| c == b' ').next_tuple().unwrap();

        let dist: u16 = atoi::atoi(raw_dist).unwrap();

        let end: [usize; 2] = [l0, l1].map(|l| {
            cities
                .iter()
                .position(|&city| city == l)
                .unwrap_or_else(|| {
                    cities.push(l);
                    cities.len() - 1
                })
        });

        for (i, j) in [(0, 1), (1, 0)] {
            dists[end[i] * num_cities + end[j]] = dist;
        }
    }

    (0..num_cities)
        .into_par_iter()
        .map(|start| {
            (0..num_cities)
                .filter(|&c| c != start)
                .permutations(num_cities - 1)
                .map(|perm| {
                    dists[start * num_cities + perm[0]]
                        + perm
                            .array_windows()
                            .map(|[l0, l1]| dists[l0 * num_cities + l1])
                            .sum::<u16>()
                })
                .fold([u16::MAX, 0], |[lowest, highest], item| {
                    [lowest.min(item), highest.max(item)]
                })
        })
        .reduce(
            || [u16::MAX, 0],
            |acc, local| [acc[0].min(local[0]), acc[1].max(local[1])],
        )
        .map(Into::into)
}
