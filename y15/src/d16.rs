use common::{FromUnsigned, PartAnswer};
use core::array;
use itertools::Itertools;

const CHILDREN: u8 = 3;
const CATS: u8 = 7;
const SAMOYEDS: u8 = 2;
const POMERANIANS: u8 = 3;
const AKITAS: u8 = 0;
const VIZSLAS: u8 = 0;
const GOLDFISH: u8 = 5;
const TREES: u8 = 3;
const CARS: u8 = 2;
const PERFUMES: u8 = 1;

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let iter_sues = || {
        input
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut words = line.split(|&c| c == b' ').skip(2);

                let mut checks: [_; 3] = array::from_fn(|_| words.next_array().unwrap());
                for count_raw in checks[0..2].iter_mut().map(|arr| &mut arr[1]) {
                    *count_raw = &count_raw[0..count_raw.len() - 1];
                }

                checks.map(|[item, count_raw]| (item, u8::from_unsigned(count_raw)))
            })
            .enumerate()
    };

    let (a_idx, _) = iter_sues()
        .find(|(_, checks)| {
            checks.iter().all(|(item, count)| {
                *count
                    == match (item[2], item[3]) {
                        (b'e', _) => TREES,
                        (b'l', _) => GOLDFISH,
                        (b't', _) => CATS,
                        (b'z', _) => VIZSLAS,
                        (b'i', b'l') => CHILDREN,
                        (b'i', b't') => AKITAS,
                        (b'm', b'e') => POMERANIANS,
                        (b'm', b'o') => SAMOYEDS,
                        (b'r', b'f') => PERFUMES,
                        (b'r', b's') => CARS,
                        _ => unreachable!(),
                    }
            })
        })
        .unwrap();

    let (b_idx, _) = iter_sues()
        .find(|(_, checks)| {
            checks.iter().all(|(item, count)| match (item[2], item[3]) {
                (b'e', _) => *count > TREES,
                (b'l', _) => *count < GOLDFISH,
                (b't', _) => *count > CATS,
                (b'z', _) => *count == VIZSLAS,
                (b'i', b'l') => *count == CHILDREN,
                (b'i', b't') => *count == AKITAS,
                (b'm', b'e') => *count < POMERANIANS,
                (b'm', b'o') => *count == SAMOYEDS,
                (b'r', b'f') => *count == PERFUMES,
                (b'r', b's') => *count == CARS,
                _ => unreachable!(),
            })
        })
        .unwrap();

    [(a_idx + 1).into(), (b_idx + 1).into()]
}
