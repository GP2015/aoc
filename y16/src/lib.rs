#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
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

use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut};
use itoa::{Buffer, Integer};

macro_rules! day_runs {
    ($($module:ident),+ $(,)?) => {
        [$($module::run),+]
    };
}

#[must_use]
#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct PartAnswer(ArrayVec<u8, 20>);

impl<T: Integer> From<T> for PartAnswer {
    fn from(value: T) -> Self {
        let mut buf = Buffer::new();
        let bytes = buf.format(value).as_bytes();
        let mut arr = Self(ArrayVec::new());
        arr.try_extend_from_slice(bytes).unwrap();
        arr
    }
}

type DayFn = fn(input: &[u8]) -> [PartAnswer; 2];

#[must_use]
pub fn days() -> [DayFn; HIGHEST_DAY] {
    days!(day_runs)
}
