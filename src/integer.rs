use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};
use std::{i8, i16, i32, u8, u16, u32};

macro_rules! int_impl {
    ($t:ident) => {
        impl APerElement for $t {
            type Result = $t;
            const TAG: u32 = 0xBEEF;
            const CONSTRAINTS: Constraints = Constraints {
                value: None,
                size: None,
            };
            fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
                let ret = decoder.decode_int(Some($t::MIN as i64), Some($t::MAX as i64));
                if ret.is_err() {
                    return Err(ret.err().unwrap());
                }
                Ok(ret.unwrap() as $t)
            }
        }
    };
}

int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
