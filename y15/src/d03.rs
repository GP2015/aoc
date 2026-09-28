use crate::common::{ArrayVecConvert, IsDay, MAX_OUTPUT_LEN};
use arrayvec::ArrayVec;
use core::array;
use rustc_hash::{FxBuildHasher, FxHashSet};

pub struct Day;

impl IsDay for Day {
    fn run(input: &[u8]) -> [ArrayVec<u8, MAX_OUTPUT_LEN>; 2] {
        let def_hash_set = || FxHashSet::with_capacity_and_hasher(input.len() + 1, FxBuildHasher);
        let [mut solo_set, mut dual_set] = array::from_fn(|_| def_hash_set());

        solo_set.insert((0, 0));
        dual_set.insert((0, 0));

        let mut solo_pos = (0, 0);
        let mut dual_pos_real = (0, 0);
        let mut dual_pos_robo = (0, 0);

        for r in input.chunks(2) {
            let step = |set: &mut FxHashSet<_>, pos: &mut (_, _), c| {
                match c {
                    b'<' => pos.0 -= 1,
                    b'>' => pos.0 += 1,
                    b'v' => pos.1 -= 1,
                    _ => pos.1 += 1, // b'^'
                }

                set.insert(*pos);
            };

            step(&mut solo_set, &mut solo_pos, r[0]);
            step(&mut solo_set, &mut solo_pos, r[1]);
            step(&mut dual_set, &mut dual_pos_real, r[0]);
            step(&mut dual_set, &mut dual_pos_robo, r[1]);
        }

        [solo_set, dual_set].map(|set| ArrayVec::from_int(set.len()))
    }
}
