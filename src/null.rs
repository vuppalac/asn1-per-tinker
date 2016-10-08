use aper::{APerElement, Constraints, Decoder, DecodeError, Encoding, EncodeError};

impl APerElement for () {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read `()` from an aligned PER encoding.
    fn from_aper(_: &mut Decoder, _: Constraints) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn to_aper(&self, _: Constraints) -> Result<Encoding, EncodeError> {
        Ok(Encoding::new())
    }
}
