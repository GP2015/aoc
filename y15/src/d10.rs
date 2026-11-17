use alloc::{boxed::Box, vec::Vec};
use common::PartAnswer;

fn generate(v: &[u8]) -> Box<[u8]> {
    let mut out = Vec::new();

    let mut dig = v[0];
    let mut len = 1;
    let mut idx = 1;

    while idx < v.len() {
        if v[idx] != dig {
            out.push(len);
            out.push(dig);

            dig = v[idx];
            len = 0;
        }

        len += 1;
        idx += 1;
    }

    out.push(len);
    out.push(dig);

    out.into_boxed_slice()
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let mut current: Box<_> = input.iter().map(|&c| c - b'0').collect();

    for _ in 0..40 {
        current = generate(&current);
    }

    let a = current.len().into();

    for _ in 0..10 {
        current = generate(&current);
    }

    [a, current.len().into()]
}
