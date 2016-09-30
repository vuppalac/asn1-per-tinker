mod decoder;
pub use self::decoder::{Decoder, DecodeError};

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct Constraints {
    pub value: Option<Constraint>,
    pub size: Option<Constraint>,
}

pub const UNCONSTRAINED: Constraints = Constraints {
    value: None,
    size: None,
};

pub trait APerElement {
    type Result;
    const TAG: u32;
    const CONSTRAINTS: Constraints; // visible constraints
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, decoder::DecodeError>;
}
