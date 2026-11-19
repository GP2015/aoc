#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;
extern crate std;

use alloc::{borrow::ToOwned, boxed::Box, format, string::String, vec::Vec};
use arrayvec::ArrayVec;
use core::{fmt::NumBuffer, time::Duration};
use derive_more::{Deref, DerefMut};
use std::{env, fs, path::Path, time::SystemTime};

#[must_use]
#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct PartAnswer(pub ArrayVec<u8, 20>);

macro_rules! impl_part_answer {
    ($($t:ident),* $(,)?) => {$(
        impl From<$t> for PartAnswer {
            fn from(value: $t) -> Self {
                let mut buf = NumBuffer::new();
                let bytes = value.format_into(&mut buf).as_bytes();
                let mut arr = Self(ArrayVec::new());
                arr.try_extend_from_slice(bytes).unwrap();
                arr
            }
        })*
    };
}
impl_part_answer!(u8, u16, u32, usize, i8, i16, i32, isize);

#[macro_export]
macro_rules! day_mods {
    ($($module:ident),+ $(,)?) => {
        $(pub mod $module;)+
    };
}

#[macro_export]
macro_rules! day_runs {
    ($($module:ident),+ $(,)?) => {
        [$($module::run),+]
    };
}

pub type DayFn = fn(input: &[u8]) -> [PartAnswer; 2];

#[must_use]
pub fn load_input(data_dir: &str, day_num: usize) -> Vec<u8> {
    let name = format!("{day_num:02}.txt");
    let path = Path::new(&data_dir).join("input").join(name);

    let mut read = fs::read(path).unwrap();
    if *read.last().unwrap() == b'\n' {
        read.pop();
    }
    read
}

#[must_use]
pub fn load_answers(data_dir: &str, day_num: usize) -> [String; 2] {
    ['a', 'b']
        .map(|part| {
            let name = format!("{day_num:02}{part}.txt");
            let path = Path::new(&data_dir).join("output").join(name);
            fs::read(path).unwrap()
        })
        .map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DayResult {
    pub day: usize,
    pub time: Duration,
    pub answers: [String; 2],
}

pub fn results(day_fns: &[DayFn]) -> Box<[DayResult]> {
    let data_dir = env::args().nth(1).unwrap();

    day_fns
        .iter()
        .enumerate()
        .map(|(day_index, day_fn)| {
            let day = day_index + 1;

            let input = load_input(&data_dir, day);
            let answers = load_answers(&data_dir, day);

            let timer = SystemTime::now();
            let output_raw = day_fn(&input);
            let time = timer.elapsed().unwrap();

            let output = output_raw.map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned());
            assert_eq!(output, answers);

            DayResult { day, time, answers }
        })
        .collect()
}
