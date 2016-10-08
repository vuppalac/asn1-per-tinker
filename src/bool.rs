use aper::{APerElement, Constraints, Decoder, DecodeError, Encoding, EncodeError};

impl APerElement for bool {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read a `bool` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, _: Constraints) -> Result<Self, DecodeError> {
        let ret = decoder.read(1);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        }
        Ok(ret.unwrap() > 0)
    }

    fn to_aper(&self, _: Constraints) -> Result<Encoding, EncodeError> {
        Ok(Encoding::with_bytes_and_padding(vec![(*self as u8) << 7], 7))
    }
}
