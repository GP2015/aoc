use crate::PartAnswer;
use core::{
    hint,
    sync::atomic::{AtomicU32, Ordering},
};
use itoa::Buffer;
use md5::{Digest, Md5};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const B_CHECK_FREQ: u32 = 1 << 6;

fn core_search(k: u32, num_cores: u32, stem_hasher: &Md5, a: &AtomicU32, b: &AtomicU32) {
    let mut num = k;
    let k_mod = k % B_CHECK_FREQ;

    loop {
        let mut num_buf = Buffer::new();
        let num_str = num_buf.format(num);

        let mut hasher = stem_hasher.clone();
        hasher.update(num_str);

        #[allow(clippy::indexing_slicing)]
        let hash: [u8; 3] = hasher.finalize()[..3].try_into().unwrap();

        if hash[0] == 0 && hash[1] == 0 {
            hint::cold_path();

            if hash[2] == 0 {
                b.fetch_min(num, Ordering::Relaxed);
                return;
            } else if hash[2] < 0x10 {
                a.fetch_min(num, Ordering::Relaxed);
            }
        }

        num += num_cores;

        if num % B_CHECK_FREQ == k_mod && num >= b.load(Ordering::Relaxed) {
            hint::cold_path();
            return;
        }
    }
}

pub fn run(input: &[u8]) -> [PartAnswer; 2] {
    let stem_hasher = Md5::new_with_prefix(input);

    let a = AtomicU32::new(u32::MAX);
    let b = AtomicU32::new(u32::MAX);

    #[allow(clippy::cast_possible_truncation)]
    let num_cores = rayon::current_num_threads() as u32;

    (0..num_cores).into_par_iter().for_each(|k| {
        core_search(k, num_cores, &stem_hasher, &a, &b);
    });

    [a, b].map(|sem| sem.load(Ordering::Relaxed).into())
}
