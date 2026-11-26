#![no_std]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;
extern crate std;

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
            d12,
            d13,
            d14,
            d15,
        }
    };
}

use common::{DayFn, day_mods, day_runs};

days!(day_mods);

#[must_use]
pub fn days() -> [DayFn; days!(day_runs).len()] {
    days!(day_runs)
}
