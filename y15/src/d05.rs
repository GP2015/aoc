use crate::PartAnswer;
use core::iter;

fn has_three_vowels(line: &[u8; 16]) -> bool {
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

fn has_pair(line: &[u8; 16]) -> bool {
    line.array_windows::<2>().any(|b| b[0] == b[1])
}

fn no_banned_pats(line: &[u8; 16]) -> bool {
    !line.array_windows::<2>().any(|window| {
        matches!(
            window,
            [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']
        )
    })
}

fn has_pat_dupe(line: &[u8; 16]) -> bool {
    let mut seen = [false; 26 * 26];
    let mut last_idx = usize::MAX;
    let mut check_last_idx = true;

    for b in line.array_windows::<2>() {
        let idx = (b[0] - b'a') as usize * 26 + (b[1] - b'a') as usize;

        #[allow(clippy::indexing_slicing)]
        let seen_idx = &mut seen[idx];

        if (!check_last_idx || idx != last_idx) && *seen_idx {
            return true;
        }

        *seen_idx = true;
        check_last_idx = idx != last_idx;
        last_idx = idx;
    }

    false
}

fn has_tri_pat(line: &[u8; 16]) -> bool {
    line.array_windows::<3>().any(|b| b[0] == b[2])
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let (chunks, end_chunk) = input.as_chunks::<17>();

    chunks
        .iter()
        .map(|chunk| chunk[..16].try_into().unwrap())
        .chain(iter::once(end_chunk.try_into().unwrap()))
        .map(|line| {
            let a = has_pair(line) && no_banned_pats(line) && has_three_vowels(line);
            let b = has_tri_pat(line) && has_pat_dupe(line);
            [u32::from(a), u32::from(b)]
        })
        .fold([0, 0], |acc, e| [acc[0] + e[0], acc[1] + e[1]])
        .map(Into::into)
}
