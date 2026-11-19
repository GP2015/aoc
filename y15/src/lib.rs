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
            d11,
        }
    };
}

days!(day_mods);

use common::{DayFn, day_mods, day_runs};

#[must_use]
pub fn days() -> [DayFn; days!(day_runs).len()] {
    days!(day_runs)
}
