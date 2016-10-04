use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};

impl APerElement for bool {
    type Result = bool;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read a `bool` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, DecodeError> {
        let ret = decoder.read(1);
        if ret.is_err() {
            return Err(DecodeError::Dummy); // XXX: meaningful error here
        }
        Ok(ret.unwrap() > 0)
    }
}
