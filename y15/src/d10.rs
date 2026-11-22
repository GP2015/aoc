use alloc::{boxed::Box, vec::Vec};
use common::PartAnswer;
use derive_more::{Deref, DerefMut};

#[derive(Clone, Debug, Deref, DerefMut, Hash, Eq, PartialEq)]
struct Sequence(Box<[u8]>);

impl Sequence {
    fn next_seq(&mut self) {
        let mut out = Vec::with_capacity(2 * self.len());

        let mut dig = self[0];
        let mut len = 1;
        let mut idx = 1;

        while idx < self.len() {
            if self[idx] != dig {
                out.push(len);
                out.push(dig);

                dig = self[idx];
                len = 0;
            }

            len += 1;
            idx += 1;
        }

        out.push(len);
        out.push(dig);

        self.0 = out.into_boxed_slice();
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut seq = Sequence(input.iter().map(|&c| c - b'0').collect());

    for _ in 0..40 {
        seq.next_seq();
    }

    let a = seq.len().into();

    for _ in 0..10 {
        seq.next_seq();
    }

    [a, seq.len().into()]
}
