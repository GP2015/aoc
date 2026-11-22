#![no_std]
extern crate alloc;
extern crate std;

use common::YearSummary;
use core::cmp::Reverse;
use mimalloc::MiMalloc;
use std::{env, println};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let data_dir = env::args().nth(1).unwrap();
    let mut summary = YearSummary::generate(&data_dir, &y16::days());
    summary.sort_by_key(|res| Reverse(res.time));
    println!("{summary}");
}
