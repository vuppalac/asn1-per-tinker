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

/// A bit-wise cursor used to decode aligned PER messagses.
///
/// # Examples
///
/// ```
/// extern crate asn1;
/// use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};
///
/// let data = b"\x80\x2b"; // 43
/// let mut d = aper::Decoder::new(data);
/// let x = i16::from_aper(&mut d, UNCONSTRAINED).unwrap();
/// println!("x = {}", x); // Prints x = 43
/// ```
pub struct Decoder<'a> {
    data: &'a [u8],
    len: usize,
    pos: usize,
}

impl<'a> Decoder<'a> {
    /// Construct a new `Decoder` with an array of bytes.
    pub fn new(data: &'a [u8]) -> Decoder {
        Decoder {
            data: data,
            len: 8 * data.len(),
            pos: 0,
        }
    }

    /// Read `n` bits. Where `0 <= n <= 8`. See [read_to_vec()](#method.read_to_vec) for larger `n`.
    /// Returns an `Err` if the read would consume more bits than are available. Else, returns the bits as a u8 with
    /// left-padding.
    ///
    /// # Examples
    ///
    /// In some cases, elements of aligned PER messages will be encoded using only the minimum number of bits required to
    /// express the value without alignment on a byte boundary. `read` allows you to decode these fields.
    ///
    /// For example, consider a bit field that only occupies three bits.  
    ///
    /// ```
    /// let data = b"\xe0";
    /// let mut d = aper::Decoder::new(data);
    /// let x = d.read(3).unwrap();
    /// println!("x = 0x{:X}"); // Prints x = 0x07
    /// ```
    pub fn read(&mut self, n: usize) -> Result<u8, ()> {
        if n == 0 {
            return Ok(0);
        }
        if self.pos + n > self.len {
            return Err(());
        }

        let mut l_bucket = self.pos / 8;
        let mut h_bucket = (self.pos + n) / 8;
        let l_off = self.pos - l_bucket * 8;
        let h_off = (self.pos + n) - h_bucket * 8;
        let mut ret: u8 = 0;

        if l_bucket == h_bucket {
            let mask = (0xFF >> (8 - n)) << (8 - h_off);
            ret = (self.data[l_bucket] & mask) >> (8 - h_off);
        } else if l_bucket < h_bucket && h_off == 0 {
            let mask = (0xFF >> (8 - n));
            ret = (self.data[l_bucket] & mask);
        } else {
            let l_mask = (0xFF >> (8 - (n - h_off)));
            let h_mask = (0xFF << (8 - h_off));
            ret = (self.data[l_bucket] & l_mask) << h_off;
            ret |= ((self.data[h_bucket] & h_mask) >> (8 - h_off))
        }
        self.pos += n;
        Ok(ret)
    }

    /// Read a byte.
    pub fn read_u8(&mut self) -> Result<u8, ()> {
        let ret = self.read(8);
        if ret.is_err() {
            return Err(());
        }
        Ok(ret.unwrap())
    }

    /// Read `len` bits into `content`.
    /// Returns an `Err` if the read would consume more bits than are available. Else, the bits as a `u8`s with
    /// left-padding are pushed onto `content`.
    ///
    /// # Examples
    ///
    /// Some fields may span multiple bytes. `read_to_vec` allows you to decode these fields.
    ///
    /// ```
    /// let data = b"\xff\xf3";
    /// let mut d = aper::Decoder::new(data);
    /// let mut x: Vec<u8> = Vec::with_capacity(2);
    /// self.read_to_vec(&mut content, 12).unwrap();
    /// println!("x = {:?}"); // Prints x = [255, 15]
    /// ```
    pub fn read_to_vec(&mut self, content: &mut Vec<u8>, len: usize) -> Result<(), ()> {
        if len == 0 {
            return Ok(());
        }
        if self.pos + len > self.len {
            return Err(());
        }

        if len < 8 {
            content.push(self.read(len).unwrap());
        } else {
            let num_bytes = (len as f64 / 8.).ceil() as usize;
            for i in 0..num_bytes {
                println!("read byte {}", i);
                content.push(self.read_u8().unwrap());
            }
            self.pos -= len % 8;
        }
        Ok(())
    }

    /// Decode an aligned PER length determinant
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

    /// Decode an Aligned PER integer between `min` and `max`
    ///
    /// You can decode the Rust primitive (u)ints: `i8`, `i16`, `i32`, `u8`, `u16`, and `u32` using their respective
    /// `from_aper` constructors. `decode_int` is useful if you want to decode an integer field that exists somewhere
    /// between or beyond the primitive widths.
    ///
    /// # Examples
    ///
    /// For example, a value in [500, 503] can be encoded using two bits in aligned PER, so using
    /// `u8` would yield an incorrect value. The code below demonstrates how to decode such a field.
    ///
    /// ```
    /// let data = b"\x70"; // 0111 0000
    /// let mut d = aper::Decoder::new(data);
    /// let x = d.decode_int(Some(500), Some(503)).unwrap();
    /// let y = d.decode_int(Some(500), Some(503)).unwrap();
    /// println!("x = {}", x); // Prints x = 501 
    /// println!("y = {}", y); // Prints y = 503
    /// ```
    pub fn decode_int(&mut self, min: Option<i64>, max: Option<i64>) -> Result<i64, DecodeError> {
        if min.is_some() && max.is_some() {
            // constrained
            let l = min.unwrap();
            let h = max.unwrap();
            let range = h - l + 1;
            let n_bits = (range as f64).log2().ceil() as usize;

            if n_bits < 8 {
                let ret = self.read(n_bits);
                if ret.is_err() {
                    return Err(DecodeError::Dummy); // XXX: meaningful error here
                }
                return Ok(ret.unwrap() as i64 + l);
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
            let res = self.read_to_vec(&mut content, len * 8);
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
        let res = self.read_to_vec(&mut content, len * 8);
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
}
