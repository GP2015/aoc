use alloc::{borrow::ToOwned, format, string::String, vec::Vec};
use core::fmt::{Display, Formatter, Result};
use derive_more::{Deref, DerefMut};
use std::{fs, path::Path};

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct Input(pub Vec<u8>);

impl Input {
    #[must_use]
    pub fn load(data_dir: &str, day_num: usize) -> Self {
        let name = format!("{day_num:02}.txt");
        let path = Path::new(&data_dir).join("input").join(name);

        let mut read = fs::read(path).unwrap();
        if *read.last().unwrap() == b'\n' {
            read.pop();
        }
        Self(read)
    }
}

#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct Answers(pub [String; 2]);

impl Answers {
    #[must_use]
    pub(crate) fn load_correct(data_dir: &str, day_num: usize) -> Self {
        Self(
            ['a', 'b']
                .map(|part| {
                    let name = format!("{day_num:02}{part}.txt");
                    let path = Path::new(&data_dir).join("output").join(name);
                    fs::read(path).unwrap()
                })
                .map(|o| str::from_utf8(o.as_slice()).unwrap().to_owned()),
        )
    }
}

impl Display for Answers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}
