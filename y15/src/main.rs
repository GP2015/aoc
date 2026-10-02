#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::indexing_slicing)]

const HIGHEST_DAY: u8 = 6;

macro_rules! days {
    ($callback:ident) => {
        $callback! {
            d01,
            d02,
            d03,
            d04,
            d05,
            d06,
        }
    };
}

extern crate alloc;
extern crate std;

macro_rules! day_mods {
    ($($module:ident),+ $(,)?) => {
        $(mod $module;)+
    };
}

mod common;
days!(day_mods);

use crate::common::{IsDay, RunInput, RunOutput};
use alloc::{borrow::ToOwned, format, vec::Vec};
use core::time::Duration;
use mimalloc::MiMalloc;
use std::{env, fs, path::Path, println, time::SystemTime};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

macro_rules! day_runs {
    ($($module:ident),+ $(,)?) => {
        [$($module::Day::run),+]
    };
}

const DAYS: [fn(&RunInput) -> RunOutput; HIGHEST_DAY as usize] = days!(day_runs);

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

    for _ in 0..1 {
        for (day_index, day_fn) in DAYS.iter().enumerate() {
            let day = day_index + 1;
            let input = input(&data_dir, day);

            let timer = SystemTime::now();
            let output_raw = day_fn(&input);
            let elapsed = timer.elapsed().unwrap();
            total += elapsed;

            let answers_raw = answers(&data_dir, day);

            let output = output_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned());
            let answers = answers_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned());

            assert_eq!(output, answers);

            println!("Day {day} - {elapsed:?} - {answers:?}");
        }
    }

    println!("Total - {total:?}");
}
