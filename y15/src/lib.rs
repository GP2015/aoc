#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;

const HIGHEST_DAY: usize = 7;

macro_rules! days {
    ($callback:ident) => {
        $callback! {
            d01,
            d02,
            d03,
            d04,
            d05,
            d06,
            d07,
        }
    };
}
macro_rules! day_mods {
    ($($module:ident),+ $(,)?) => {
        $(pub mod $module;)+
    };
}
days!(day_mods);

type DayFn = fn(input: &[u8]) -> [common::PartAnswer; 2];

macro_rules! day_runs {
    ($($module:ident),+ $(,)?) => {
        [$($module::run),+]
    };
}

#[must_use]
pub fn days() -> [DayFn; HIGHEST_DAY] {
    days!(day_runs)
}
