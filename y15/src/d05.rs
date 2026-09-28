use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use arrayvec::ArrayVec;
use core::array;

fn has_three_vowels(line: &[u8]) -> bool {
    let mut vowels = 0;

    for &byte in line {
        let has_vowel = matches!(byte, b'a' | b'e' | b'i' | b'o' | b'u');

        if has_vowel {
            vowels += 1;

            if vowels >= 3 {
                return true;
            }
        }
    }

    false
}

fn has_pair(line: &[u8]) -> bool {
    line.windows(2).any(|b| b[0] == b[1])
}

fn no_banned_pats(line: &[u8]) -> bool {
    !line.windows(2).any(|window| {
        matches!(
            window,
            [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']
        )
    })
}

fn has_pat_dupe(line: &[u8]) -> bool {
    let mut seen = [false; 26 * 26];
    let mut last_idx = usize::MAX;
    let mut check_last_idx = true;

    for b in line.windows(2) {
        let idx = (b[0] - b'a') as usize * 26 + (b[1] - b'a') as usize;

        if (!check_last_idx || idx != last_idx) && seen[idx] {
            return true;
        }

        seen[idx] = true;
        check_last_idx = idx != last_idx;
        last_idx = idx;
    }

    false
}

fn has_tri_pat(line: &[u8]) -> bool {
    line.windows(3).any(|b| b[0] == b[2])
}

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        input
            .chunks(17)
            .map(|line_raw| {
                let line = &line_raw[0..16];
                let a = has_pair(line) && no_banned_pats(line) && has_three_vowels(line);
                let b = has_tri_pat(line) && has_pat_dupe(line);
                [u32::from(a), u32::from(b)]
            })
            .fold([0, 0], |acc, e| array::from_fn(|i| acc[i] + e[i]))
            .map(ArrayVec::from_int)
    }
}
