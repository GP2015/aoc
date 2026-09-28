use arrayvec::ArrayVec;
use itoa::{Buffer, Integer};

pub const MAX_OUTPUT_LEN: usize = 20;

pub type RunInput = [u8];
pub type RunOutput = [ArrayVec<u8, MAX_OUTPUT_LEN>; 2];

pub trait IsDay {
    fn run(input: &RunInput) -> RunOutput;
}

pub trait ArrayVecConvert {
    fn from_int(num: impl Integer) -> Self;
    // fn from_ascii(bytes: &[u8]) -> Self;
}

impl<const SIZE: usize> ArrayVecConvert for ArrayVec<u8, SIZE> {
    fn from_int(num: impl Integer) -> Self {
        let mut buf = Buffer::new();
        let bytes = buf.format(num).as_bytes();
        let mut arr = Self::new();
        arr.try_extend_from_slice(bytes).unwrap();
        arr
    }

    // fn from_ascii(bytes: &[u8]) -> Self {
    //     let mut arr = Self::new();
    //     arr.try_extend_from_slice(bytes).unwrap();
    //     arr
    // }
}
