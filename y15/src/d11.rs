use arrayvec::ArrayVec;
use common::PartAnswer;
use derive_more::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
struct Password([u8; 8]);

impl Password {
    fn increment(&mut self) {
        let mut idx = 7;

        loop {
            if self[idx] != b'z' {
                self[idx] += 1;
                return;
            }

            self[idx] = b'a';
            idx -= 1;
        }
    }

    fn has_two_pairs(&self) {
        //
    }

    fn is_valid(self) -> bool {
        let has_straight = self
            .array_windows::<3>()
            .any(|c| c[0] <= b'x' && c[1] == c[0] + 1 && c[2] == c[1] + 1);

        let no_banned_letters = !self.iter().any(|&c| b"iol".contains(&c));

        let has_two_pairs = todo!();

        has_straight && no_banned_letters && has_two_pairs
    }

    pub fn next_valid(self) -> Self {
        let mut out = self;

        loop {
            out.increment();

            if out.is_valid() {
                return out;
            }
        }
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
