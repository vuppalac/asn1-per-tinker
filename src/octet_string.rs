use aper::{APerElement, Constraint, Constraints, Decoder, DecodeError};

impl APerElement for Vec<u8> {
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
            return Ok(Vec::new());
        }

        let len = sz_constr.max().unwrap() as usize;
        if len >= 65535 {
            unimplemented!();
        }

        let mut content: Vec<u8> = Vec::with_capacity(len);
        let ret = decoder.read_to_vec(&mut content, len);
        if ret.is_err() {
            return Err(DecodeError::Dummy); // XXX: meaningful error here
        }

        Ok(content)
    }
}
