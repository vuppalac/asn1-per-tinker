mod decoder;
pub use self::decoder::{Decoder, DecodeError};

/// An interval that desribes the limits on some value.
/// To indicate something is unbounded, set `min` and `max` to `None`.
#[derive(Debug, Copy, Clone)]
pub struct Constraint {
    min: Option<i64>,
    max: Option<i64>,
}

impl Constraint {
    /// Construct a new `Constraint`.
    pub fn new(min: Option<i64>, max: Option<i64>) -> Constraint {
        Constraint {
            min: min,
            max: max,
        }
    }

    /// Get the lower bound.
    pub fn min(&self) -> Option<i64> {
        self.min
    }

    /// Get the upper bound.
    pub fn max(&self) -> Option<i64> {
        self.max
    }
}

/// A pair of `Constraint`s that describes the constraints on the value (if applicable) and encoded size of a type.
/// A value is considered unconstrained if `value` and `size` are both set to `None`.
#[derive(Debug, Copy, Clone)]
pub struct Constraints {
    pub value: Option<Constraint>,
    pub size: Option<Constraint>,
}

pub const UNCONSTRAINED: Constraints = Constraints {
    value: None,
    size: None,
};

/// Trait for Aligned PER encoding/decoding.
pub trait APerElement {
    /// The type to be returned by `from_aper`, usually `Self`
    type Result;

    /// A tag for determining canonical-PER ordering. Not currently used.
    const TAG: u32;

    /// PER-visible Constraints
    const CONSTRAINTS: Constraints;

    /// Constructor for the `Result` type given an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, decoder::DecodeError>;
}
