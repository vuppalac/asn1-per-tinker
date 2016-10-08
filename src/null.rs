use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError, Encoding, EncodeError};

impl APerElement for () {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read `()` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        Ok(Encoding::new())
    }
}
