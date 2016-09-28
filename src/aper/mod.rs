mod decoder;
pub use self::decoder::{Decoder, DecodeError};

pub struct Constraint {
    min: Option<i64>,
    max: Option<i64>,
}

impl Constraint {
    pub fn new(min: Option<i64>, max: Option<i64>) -> Constraint {
        Constraint {
            min: min,
            max: max,
        }
    }

    pub fn min(&self) -> Option<i64> {
        self.min
    }

    pub fn max(&self) -> Option<i64> {
        self.max
    }
}

pub struct Constraints {
    pub value: Option<Constraint>,
    pub size: Option<Constraint>,
}

pub trait APerElement {
    type Result;
    const TAG: u32;
    const CONSTRAINTS: Constraints; // visible constraints
    fn aper_decode(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, decoder::DecodeError>;
}
