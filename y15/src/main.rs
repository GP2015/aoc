#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
extern crate alloc;
extern crate std;

use core::time::Duration;
use mimalloc::MiMalloc;
use std::println;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let results = common::results(&y15::days());
    // results.sort_by_key(|res| Reverse(res.time));

    for r in &results {
        println!("Day {} - {:?} - {:?}", r.day, r.time, r.answers);
    }

    let total: Duration = results.iter().map(|r| r.time).sum();
    println!("Total - {total:?}");
}
