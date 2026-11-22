use arrayvec::ArrayVec;
use common::PartAnswer;
use derive_more::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Password([u8; 8]);

impl Password {
    fn has_two_pairs(self) -> bool {
        let mut found_one = false;
        let mut skip = false;

        for w in self.array_windows::<2>() {
            if skip {
                skip = false;
                continue;
            }

            if w[0] == w[1] {
                if found_one {
                    return true;
                }

                found_one = true;
                skip = true;
            }
        }

        false
    }

    fn has_straight(self) -> bool {
        self.array_windows::<3>()
            .any(|c| c[0] <= b'x' && c[1] == c[0] + 1 && c[2] == c[1] + 1)
    }

    fn is_valid(self) -> bool {
        self.has_straight() && self.has_two_pairs()
    }

    fn next_attempt(mut self) -> Self {
        for idx in (0..8).rev() {
            if b"hnk".contains(&self[idx]) {
                self[idx] += 1;

                for c in &mut self[idx + 1..8] {
                    *c = b'a';
                }

                break;
            }

            if self[idx] != b'z' {
                self[idx] += 1;
                break;
            }

            self[idx] = b'a';
        }

        self
    }

    pub fn next_valid(mut self) -> Self {
        self = self.next_attempt();

        while !self.is_valid() {
            self = self.next_attempt();
        }

        self
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let password = Password(input.try_into().unwrap());
    let a = password.next_valid();
    let b = a.next_valid();

    [a, b].map(|r| {
        let mut out = ArrayVec::new();
        out.try_extend_from_slice(&*r).unwrap();
        PartAnswer(out)
    })
}
