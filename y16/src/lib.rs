#![no_std]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;

const HIGHEST_DAY: usize = 1;

macro_rules! days {
    ($callback:ident) => {
        $callback! {
            d01,
        }
    };
}
macro_rules! day_mods {
    ($($module:ident),+ $(,)?) => {
        $(pub mod $module;)+
    };
}
days!(day_mods);

macro_rules! day_runs {
    ($($module:ident),+ $(,)?) => {
        [$($module::run),+]
    };
}

type DayFn = fn(input: &[u8]) -> [common::PartAnswer; 2];

#[must_use]
pub fn days() -> [DayFn; HIGHEST_DAY] {
    days!(day_runs)
}
