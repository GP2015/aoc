use bytemuck::{Pod, Zeroable};
use common::{PartAnswer, md5};
use core::{
    fmt::NumBuffer,
    hint,
    sync::atomic::{AtomicU32, Ordering},
};
use crunchy::unroll;
use derive_more::{Deref, DerefMut};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq, Pod, Zeroable)]
pub struct PaddedMessage([u8; 64]);

#[allow(clippy::many_single_char_names)]
fn hash_start(mut padded: PaddedMessage, num: u32) -> [u8; 3] {
    let mut num_buf = NumBuffer::new();
    let num_str = num.format_into(&mut num_buf);
    let append = num_str.as_bytes();

    let total_len = 8 + append.len();

    padded[8..total_len].copy_from_slice(append);
    padded[total_len] = 0x80;
    padded[56..64].copy_from_slice(&(total_len as u64 * 8).to_le_bytes());

    let m: &[u32; 16] = bytemuck::cast_ref(&padded);

    let mut a = md5::A0_INITIAL;
    let mut b = md5::B0_INITIAL;
    let mut c = md5::C0_INITIAL;
    let mut d = md5::D0_INITIAL;

    macro_rules! w_add {
        ($first:expr $(, $rest:expr)* $(,)?) => {
            $first $(.wrapping_add($rest))*
        };
    }

    unroll! {
        for i in 0..64 {
            let f = match i {
                0..16 => (b & c) | (!b & d),
                16..32 => (d & b) | (!d & c),
                32..48 => b ^ c ^ d,
                48..64 => c ^ (b | !d),
                _ => unreachable!()
            };

            let g = match i {
                0..16 => i,
                16..32 => (5 * i + 1) % 16,
                32..48 => (3 * i + 5) % 16,
                48..64 => (7 * i) % 16, // 48..64
                _ => unreachable!()
            };

            let f = w_add!(f, a, md5::K[i], m[g]);
            a = d;

            #[allow(unused)]
            {
                d = c;
                c = b;
                b = w_add!(b, f.rotate_left(md5::S[i]));
            }
        }
    }

    w_add!(md5::A0_INITIAL, a).to_le_bytes()[..3]
        .try_into()
        .unwrap()
}

const B_CHECK_FREQ: u32 = 1 << 6;

fn core_search(
    k: u32,
    num_cores: u32,
    partial_padded: PaddedMessage,
    a: &AtomicU32,
    b: &AtomicU32,
) {
    let mut num = k;
    let k_mod = k % B_CHECK_FREQ;

    loop {
        let hash = hash_start(partial_padded, num);

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
    let a = AtomicU32::new(u32::MAX);
    let b = AtomicU32::new(u32::MAX);

    let mut partial_padded = PaddedMessage([0; 64]);
    partial_padded[..8].copy_from_slice(input);

    #[allow(clippy::cast_possible_truncation)]
    let num_cores = rayon::current_num_threads() as u32;

    (0..num_cores).into_par_iter().for_each(|k| {
        core_search(k, num_cores, partial_padded, &a, &b);
    });

    [a, b].map(|sem| sem.load(Ordering::Relaxed).into())
}
