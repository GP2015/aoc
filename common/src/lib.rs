#![no_std]
#![allow(clippy::fallible_impl_from, clippy::missing_panics_doc)]
extern crate alloc;
extern crate std;

mod data;
pub mod md5;
mod num_parse;
mod part;
mod summary;

pub use crate::{
    data::Answers,
    num_parse::{FromSigned, FromUnsigned},
    part::PartAnswer,
    summary::{DaySummary, YearSummary},
};

pub type DayFn = fn(input: &[u8]) -> [PartAnswer; 2];

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
