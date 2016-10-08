use aper::{APerElement, Constraints, Decoder, DecodeError, Encoding, EncodeError, encode_length};

impl<T: APerElement> APerElement for Vec<T> {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: None,
    };

    /// Read a `Vec[T]` from an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if constraints.size.is_none() {
            return Err(DecodeError::MissingSizeConstraint);
        }
        let sz_constr = constraints.size.unwrap();

        let mut min_len: usize = 0;
        let mut max_len: usize = 0;
        if sz_constr.min().is_some() {
            min_len = sz_constr.min().unwrap() as usize;
        }
        if sz_constr.max().is_some() {
            max_len = sz_constr.max().unwrap() as usize;
        }

        if max_len >= 65535 {
            return Err(DecodeError::NotImplemented);
        }

        let len: usize;
        if max_len == min_len {
            len = max_len;
        } else {
            let ret = decoder.decode_length();
            if ret.is_err() {
                return Err(ret.err().unwrap());
            }
            len = ret.unwrap();
        }

        // XXX: This is terrible, but convenient. Either fix or document thoroughly.
        let el_constrs = Constraints {
            value: None,
            size: constraints.value,
        };
        let mut content: Vec<T> = Vec::with_capacity(len);
        for _ in 0..len {
            let ret = T::from_aper(decoder, el_constrs);
            if ret.is_err() {
                return Err(ret.err().unwrap());
            }
            content.push(ret.unwrap());
        }

        Ok(content)
    }

    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let ret = encode_length(self.len());
        if ret.is_err() {
            return Err(ret.err().unwrap());
        }
        let mut enc = ret.unwrap();
        for x in self {
            let ret = enc.append(&x.to_aper(Constraints {
                    value: None,
                    size: constraints.value,
                })
                .unwrap());
            if ret.is_err() {
                return Err(ret.err().unwrap());
            }
        }
        Ok(enc)
    }
}
