use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut};
use itoa::{Buffer, Integer};

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
