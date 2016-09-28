mod decoder;
pub use self::decoder::{Decoder, DecodeError};

#[derive(PartialEq)]
pub enum ConstraintFlags {
    UNCONSTRAINED = 0x0,
    SEMI_CONSTRAINED = 0x1,
    CONSTRAINED = 0x2,
    EXTENSIBLE = 0x3,
}

pub struct Constraint {
    flags: ConstraintFlags,
    range_bits: u32,
    effective_bits: u32,
    min: i64,
    max: i64,
}

pub struct Constraints {
    value: Constraint,
    size: Constraint,
}

pub trait APerElement {
    type Result;
    const TAG: u32;
    const CONSTRAINTS: Option<Constraints>; // visible constraints
    fn aper_decode(data: &[u8]) -> Result<Self::Result, decoder::DecodeError>;
}
