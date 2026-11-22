use common::PartAnswer;
use derive_more::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Line<'a>(&'a [u8]);

impl Line<'_> {
    fn has_three_vowels(&self) -> bool {
        let mut vowels = 0;

        for &byte in self.iter() {
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

    pub fn a_valid(&self) -> bool {
        let has_pair = self.array_windows::<2>().any(|w| w[0] == w[1]);

        let no_banned_pats = !self
            .array_windows::<2>()
            .any(|window| matches!(window, b"ab" | b"cd" | b"pq" | b"xy"));

        has_pair && no_banned_pats && self.has_three_vowels()
    }

    fn has_pat_dupe(&self) -> bool {
        let mut seen = [false; 26 * 26];
        let mut last_idx = usize::MAX;
        let mut check_last_idx = true;

        for w in self.array_windows::<2>() {
            let idx = (w[0] - b'a') as usize * 26 + (w[1] - b'a') as usize;

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

    pub fn b_valid(&self) -> bool {
        let has_tri_pat = self.array_windows::<3>().any(|w| w[0] == w[2]);
        has_tri_pat && self.has_pat_dupe()
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            let line = Line(line);
            [line.a_valid(), line.b_valid()].map(u32::from)
        })
        .fold([0, 0], |acc, e| [acc[0] + e[0], acc[1] + e[1]])
        .map(Into::into)
}
