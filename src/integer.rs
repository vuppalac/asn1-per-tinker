use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError, Encoding, EncodeError, encode_int};
use std::{i8, i16, i32, u8, u16, u32};

macro_rules! int_impl {
    ($t:ident) => {
        impl APerElement for $t {
            const CONSTRAINTS: Constraints = Constraints {
                value: None,
                size: None,
            };
            /// Read an `$t` from an aligned PER encoding.
            fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
                let ret = decoder.decode_int(Some($t::MIN as i64), Some($t::MAX as i64));
                if ret.is_err() {
                    return Err(ret.err().unwrap());
                }
                Ok(ret.unwrap() as $t)
            }

            fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
                let ret = encode_int(*self as i64, Some($t::MIN as i64), Some($t::MAX as i64));
                if ret.is_err() {
                    return Err(ret.err().unwrap());
                }
                Ok(ret.unwrap())
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
