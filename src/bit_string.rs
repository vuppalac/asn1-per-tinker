use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};
use std::cmp;

#[derive(Debug)]
pub struct BitString {
    data: Vec<u8>,
    num_bits: usize,
}

impl BitString {
    pub fn with_len(n: usize) -> BitString {
        let mut ret = BitString {
            data: Vec::<u8>::with_capacity(n / 8),
            num_bits: 0,
        };
        ret.set_num_bits(n);
        ret
    }

    pub fn with_bytes_and_len(data: &Vec<u8>, n: usize) -> BitString {
        BitString {
            data: data.clone(),
            num_bits: n,
        }
    }

    pub fn get_num_bits(&self) -> usize {
        self.num_bits
    }

    pub fn set_num_bits(&mut self, n: usize) {
        self.num_bits = n;
        self.data.resize(n, 0);
    }

    pub fn is_set(&self, i: usize) -> bool {
        let mut bucket = i / 8;
        let mut pos = (i as i64 - bucket as i64 * 8) as usize;
        if bucket > self.data.len() {
            return false;
        }

        bucket = cmp::max(0, self.data.len() as i64 - bucket as i64 - 1) as usize;
        (self.data[bucket] & (1 << pos)) > 0
    }

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

    fn aper_decode(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
        if constraints.size.is_none() {
            return Err(DecodeError::Dummy); // XXX: meaningful error here
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
        let ret = decoder.read_to_vec(&mut content, num_bytes);
        if ret.is_err() {
            return Err(DecodeError::Dummy); // XXX: meaningful error here
        }

        let delta = num_bytes * 8 - len;
        if delta > 0 && num_bytes > 1 {
            shift_bytes(&mut content, delta);
        }

        Ok(BitString::with_bytes_and_len(&content, len))
    }
}
