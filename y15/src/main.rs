#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
extern crate alloc;
extern crate std;

use alloc::{borrow::ToOwned, format, vec::Vec};
use core::time::Duration;
use mimalloc::MiMalloc;
use std::{env, fs, path::Path, println, time::SystemTime};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn input(data_dir: &str, day_num: usize) -> Vec<u8> {
    let name = format!("{day_num:02}.txt");
    let path = Path::new(&data_dir).join("input").join(name);
    fs::read(path).unwrap()
}

fn answers(data_dir: &str, day_num: usize) -> [Vec<u8>; 2] {
    ['a', 'b'].map(|part| {
        let name = format!("{day_num:02}{part}.txt");
        let path = Path::new(&data_dir).join("output").join(name);
        fs::read(path).unwrap()
    })
}

fn main() {
    let data_dir = env::args().nth(1).unwrap();

    let mut total = Duration::default();
    for (day_index, day_fn) in y15::days().iter().enumerate() {
        let day = day_index + 1;

        let input_raw = input(&data_dir, day);
        let input = input_raw.trim_ascii_end();

        let timer = SystemTime::now();
        let output_raw = day_fn(input);
        let elapsed = timer.elapsed().unwrap();
        total += elapsed;

        let answers_raw = answers(&data_dir, day);

        let output = output_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned());
        let answers = answers_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned());

        assert_eq!(output, answers);

        println!("Day {day} - {elapsed:?} - {answers:?}");
    }

    println!("Total - {total:?}");
}
