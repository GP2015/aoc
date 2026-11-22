use arrayvec::ArrayVec;
use core::fmt::NumBuffer;
use derive_more::{Deref, DerefMut};

#[must_use]
#[derive(Clone, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct PartAnswer(pub ArrayVec<u8, 20>);

macro_rules! impl_part_answer {
    ($($t:ident),* $(,)?) => {$(
        impl From<$t> for PartAnswer {
            fn from(value: $t) -> Self {
                let mut buf = NumBuffer::new();
                let bytes = value.format_into(&mut buf).as_bytes();
                let mut arr = Self(ArrayVec::new());
                arr.try_extend_from_slice(bytes).unwrap();
                arr
            }
        })*
    };
}
impl_part_answer!(u8, u16, u32, usize, i8, i16, i32, isize);
