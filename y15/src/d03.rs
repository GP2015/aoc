use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use arrayvec::ArrayVec;
use core::array;
use rustc_hash::{FxBuildHasher, FxHashSet};

fn step(set: &mut FxHashSet<(i32, i32)>, pos: &mut (i32, i32), c: u8) {
    match c {
        b'<' => pos.0 -= 1,
        b'>' => pos.0 += 1,
        b'v' => pos.1 -= 1,
        b'^' => pos.1 += 1,
        _ => unreachable!(),
    }

    set.insert(*pos);
}

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let (chunks, _) = input.as_chunks::<2>();

        let def_hash_set = || FxHashSet::with_capacity_and_hasher(input.len() + 1, FxBuildHasher);
        let [mut solo_set, mut dual_set] = array::from_fn(|_| def_hash_set());

        solo_set.insert((0, 0));
        dual_set.insert((0, 0));

        let mut solo_pos = (0, 0);
        let mut dual_pos_real = (0, 0);
        let mut dual_pos_robo = (0, 0);

        for chunk in chunks {
            for &instr in chunk {
                step(&mut solo_set, &mut solo_pos, instr);
            }
            step(&mut dual_set, &mut dual_pos_real, chunk[0]);
            step(&mut dual_set, &mut dual_pos_robo, chunk[1]);
        }

        [solo_set, dual_set].map(|set| ArrayVec::from_int(set.len()))
    }
}
