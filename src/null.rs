use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};

impl APerElement for () {
    type Result = ();
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read `()` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
        Ok(())
    }
}
