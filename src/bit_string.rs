use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};
use std::cmp;

/// A bit string.
///
/// # Examples
///
/// ```
/// extern crate asn1;
/// use asn1::BitString;
///
/// let mut b = BitString::with_len(64);
/// b.set(0, true);
/// println!("b[0] = {}", b.is_set(0)); // Prints b[0] = true
/// ```
#[derive(Debug)]
pub struct BitString {
    data: Vec<u8>,
    num_bits: usize,
}

impl BitString {
    /// Consturct a `BitString` of length `n` with all values set to 0.
    pub fn with_len(n: usize) -> BitString {
        let mut ret = BitString {
            data: Vec::<u8>::with_capacity(n / 8),
            num_bits: 0,
        };
        ret.set_num_bits(n);
        ret
    }

    /// Consturct a `BitString` of length `n` with initial values contained in `data`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate asn1;
    /// use asn1::BitString;
    ///
    /// let v = vec![0x00 as u8, 0x02 as u8];
    /// let b = BitString::with_bytes_and_len(&v, 15);
    /// println!("b[0] = {}", b.is_set(0)); // Prints b[0] = false
    /// println!("b[14] = {}", b.is_set(14)); // Prints b[14] = true
    /// ```
    pub fn with_bytes_and_len(data: &Vec<u8>, n: usize) -> BitString {
        BitString {
            data: data.clone(),
            num_bits: n,
        }
    }

    /// Get the length of a `BitString`
    pub fn get_num_bits(&self) -> usize {
        self.num_bits
    }

    /// Set the length of a `BitString` and initialize any new values to 0
    pub fn set_num_bits(&mut self, n: usize) {
        self.num_bits = n;
        self.data.resize(n, 0);
    }

    /// Check if bit `i` is set.
    pub fn is_set(&self, i: usize) -> bool {
        let mut bucket = i / 8;
        let mut pos = (i as i64 - bucket as i64 * 8) as usize;
        if bucket > self.data.len() {
            return false;
        }

        bucket = cmp::max(0, self.data.len() as i64 - bucket as i64 - 1) as usize;
        (self.data[bucket] & (1 << pos)) > 0
    }

    /// Set bit `i` to `val`.
    pub fn set(&mut self, i: usize, val: bool) {
        let mut bucket = i / 8;
        let mut pos = (i as i64 - bucket as i64 * 8) as usize;
        if bucket > self.data.len() {
            return;
        }

        bucket = cmp::max(0, self.data.len() as i64 - bucket as i64 - 1) as usize;
        if val {
            self.data[bucket] |= 1 << pos;
        } else {
            self.data[bucket] &= 0xFF & !(1 << pos);
        }
    }
}

fn shift_bytes(data: &mut Vec<u8>, shift: usize) {
    let mask = !(0xFF >> shift);
    let mut frag: u8 = 0x00;
    if data.len() < 1 {
        return;
    }
    data[0] <<= shift;
    for i in (1..data.len()).rev() {
        frag = data[i] & mask;
        data[i] <<= shift;
        data[i - 1] |= frag >> (8 - shift);
    }
}

impl APerElement for BitString {
    type Result = Self;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Construct a `BitString` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
        if constraints.size.is_none() {
            return Err(DecodeError::MissingSizeConstraint);
        }

        let sz_constr = constraints.size.unwrap();
        if sz_constr.max().is_none() || sz_constr.max().unwrap() == 0 {
            return Ok(BitString::with_len(0));
        }

        let len = sz_constr.max().unwrap() as usize;
        if len >= 65535 {
            unimplemented!();
        }

        let num_bytes = (len as f64 / 8.).ceil() as usize;
        let mut content: Vec<u8> = Vec::with_capacity(num_bytes);
        let ret = decoder.read_to_vec(&mut content, len);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        }

        let delta = num_bytes * 8 - len;
        if delta > 0 && num_bytes > 1 {
            shift_bytes(&mut content, delta);
        }

        Ok(BitString::with_bytes_and_len(&content, len))
    }
}
