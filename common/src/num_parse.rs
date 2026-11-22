pub trait FromUnsigned {
    fn from_unsigned(arr: &[u8]) -> Self;
}

macro_rules! unsigned_u {
    ($t:ident) => {
        impl FromUnsigned for $t {
            fn from_unsigned(arr: &[u8]) -> Self {
                arr.iter()
                    .map(|&c| c - b'0')
                    .fold(0, |acc, d| acc * 10 + $t::from(d))
            }
        }
    };
}
unsigned_u!(u8);
unsigned_u!(u16);
unsigned_u!(u32);
unsigned_u!(usize);

macro_rules! signed_u {
    ($t:ident) => {
        impl FromUnsigned for $t {
            fn from_unsigned(arr: &[u8]) -> Self {
                arr.iter()
                    .map(|&c| c - b'0')
                    .fold(0, |acc, d| acc * 10 + $t::from(d.cast_signed()))
            }
        }
    };
}
signed_u!(i8);
signed_u!(i16);
signed_u!(i32);
signed_u!(isize);

pub trait FromSigned {
    fn from_signed(arr: &[u8]) -> Self;
}

macro_rules! signed_i {
    ($t:ident) => {
        impl FromSigned for $t {
            fn from_signed(arr: &[u8]) -> Self {
                if arr[0] == b'-' {
                    -$t::from_unsigned(&arr[1..])
                } else {
                    $t::from_unsigned(arr)
                }
            }
        }
    };
}
signed_i!(i8);
signed_i!(i16);
signed_i!(i32);
signed_i!(isize);
