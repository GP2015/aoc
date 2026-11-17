#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;

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
            d08,
            d09,
            d10,
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
pub fn days() -> [DayFn; days!(day_runs).len()] {
    days!(day_runs)
}
