use byteorder::{ByteOrder, BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, BufRead, Read, Write, Cursor};
use super::*;

const LENGTH_DET_SHORT: u8 = 0b0000_0000;
const LENGTH_DET_LONG: u8 = 0b1000_0000;
const LENGTH_DET_FRAG: u8 = 0b1100_0000;

const LENGTH_MASK_SHORT: u8 = 0b0111_1111;
const LENGTH_MASK_LONG: u8 = 0b0011_1111;

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Dummy,
}

pub struct Decoder<'a> {
    cur: Cursor<&'a [u8]>,
    padding: i32,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Decoder {
        Decoder {
            cur: Cursor::new(data),
            padding: 0,
        }
    }

    // Read bits with left-padding
    fn peek_with_padding(&mut self, padding: i32) -> Result<u8, ()> {
        let ret = self.cur.read_u8();
        if ret.is_err() {
            return Err(());
        }
        let b = ret.unwrap();
        let pos = self.cur.position();
        self.cur.set_position(pos - 1);
        if padding < 0 {
            Ok(b >> -padding)
        } else {
            Ok(b << padding)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, ()> {
        if self.padding > 0 {
            let padding = self.padding;
            self.padding = -padding;
            return self.peek_with_padding(padding);
        } else if self.padding < 0 {
            let padding = self.padding;
            self.padding = 0;
            return self.peek_with_padding(padding);
        }
        let ret = self.cur.read_u8();
        if ret.is_err() {
            return Err(());
        }
        Ok(ret.unwrap())
    }

    pub fn read_to_vec(&mut self, content: &mut Vec<u8>, len: usize) -> Result<(), ()> {
        for _ in 0..len {
            let ret = self.read_u8();
            if ret.is_err() {
                return Err(());
            }
            content.push(ret.unwrap());
        }
        Ok(())
    }

    pub fn decode_length(&mut self) -> Result<usize, DecodeError> {
        let mut ret = self.read_u8();
        if ret.is_err() {
            unreachable!(); // XXX: use meaninful error code here
        }

        let mut b = ret.unwrap();
        if b & LENGTH_DET_FRAG > 0 {
            unimplemented!();
        } else if b & LENGTH_DET_LONG > 0 {
            let len: usize = (b & LENGTH_MASK_LONG) as usize;
            ret = self.read_u8();
            if ret.is_err() {
                return Err(DecodeError::Dummy); // XXX: return meaningful error here
            }
            b = ret.unwrap();
            return Ok((len << 8) + b as usize);
        }
        Ok((b & LENGTH_MASK_SHORT) as usize)
    }

    pub fn decode_int(&mut self, min: Option<i64>, max: Option<i64>) -> Result<i64, DecodeError> {
        if min.is_some() && max.is_some() {
            // constrained
            let l = min.unwrap();
            let h = max.unwrap();
            let range = h - l + 1;
            let n_bits = (range as f64).log2().ceil() as usize;

            // XXX: implement this case
            if n_bits < 8 {
                unimplemented!();
            }

            // Simple case, no length determinant
            if n_bits <= 16 {
                let mut ret = self.read_u8();
                if ret.is_err() {
                    return Err(DecodeError::Dummy); // XXX: meaningful error here
                }

                let mut b: u16 = ret.unwrap() as u16;
                if n_bits > 8 {
                    ret = self.read_u8();
                    if ret.is_err() {
                        return Err(DecodeError::Dummy); // XXX: meaningful error here
                    }
                    b = (ret.unwrap() as u16) + (b << 8);
                }
                return Ok(b as i64 + l);
            }

            // Need to decode length determinant
            let ret = self.decode_length();
            if ret.is_err() {
                return Err(DecodeError::Dummy); // XXX: meaningful error code
            }

            let len: usize = ret.unwrap();
            if len > 8 {
                unimplemented!();
            }

            let mut content: Vec<u8> = Vec::with_capacity(len);
            let res = self.read_to_vec(&mut content, len);
            if res.is_err() {
                return Err(DecodeError::Dummy); // XXX: meaningful error code
            }

            let val = BigEndian::read_uint(&content.as_slice(), len) as i64 + l;
            if val < l || val > h {
                return Err(DecodeError::Dummy); // XXX: meaningful error code
            }
            return Ok(val);
        }

        let ret = self.decode_length();
        if ret.is_err() {
            return Err(DecodeError::Dummy); // XXX: meaningful error code
        }

        let len = ret.unwrap();
        let mut content: Vec<u8> = Vec::with_capacity(len);
        let res = self.read_to_vec(&mut content, len);
        if res.is_err() {
            return Err(DecodeError::Dummy); // XXX: meaningful error code
        }

        if min.is_none() {
            // unconstrained
            Ok(BigEndian::read_int(&content, len))
        } else {
            // semiconstrained
            Ok(BigEndian::read_int(&content, len) + min.unwrap())
        }
    }

    pub fn decode<T: APerElement>(&mut self, constraints: Constraints) -> Result<T::Result, DecodeError> {
        T::aper_decode(self, constraints)
    }
}
