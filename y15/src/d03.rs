use crate::PartAnswer;
use core::array;
use rustc_hash::{FxBuildHasher, FxHashSet};

const START_POS: u32 = (1 << 31) | (1 << 15);

fn step(set: &mut FxHashSet<u32>, pos: &mut u32, c: u8) {
    match c {
        b'<' => *pos -= 1 << 16,
        b'>' => *pos += 1 << 16,
        b'^' => *pos += 1,
        _ => *pos -= 1, // b'v'
    }

    set.insert(*pos);
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let (chunks, _) = input.as_chunks::<2>();

    let def_hash_set = || FxHashSet::with_capacity_and_hasher(input.len() + 1, FxBuildHasher);
    let [mut solo_set, mut dual_set] = array::from_fn(|_| def_hash_set());

    for set in [&mut solo_set, &mut dual_set] {
        set.insert(START_POS);
    }

    let [mut solo_pos, mut dual_pos_real, mut dual_pos_robo] = [START_POS; _];

    for chunk in chunks {
        for &instr in chunk {
            step(&mut solo_set, &mut solo_pos, instr);
        }

        step(&mut dual_set, &mut dual_pos_real, chunk[0]);
        step(&mut dual_set, &mut dual_pos_robo, chunk[1]);
    }

    [solo_set.len().into(), dual_set.len().into()]
}
