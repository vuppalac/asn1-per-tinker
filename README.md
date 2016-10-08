This crate provides tools for encoding and decoding ASN.1 messages.

Currently, only the Aligned Packed Encoding Rules (APER) are supported.

# Documentation

See [https://melvinw.github.io/rust-asn1/asn1](https://melvinw.github.io/rust-asn1/asn1).

# Usage

Add the following to your `Cargo.toml`.

```rust
[dependencies]
asn1 = { git = "https://github.com/melvinw/rust-asn1" }
```

To encode/decode your own types, just implement the `APerElement` trait. Below is an example for a simple ASN.1 messsage, `foo`.

```
foo ::= SEQUENCE {
    bar BIT STRING(SIZE(4)
    baz INTEGER(0..4294967295)
}
```

```rust
#![feature(associated_consts)]
extern crate asn1;
use asn1::BitString;
use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};

struct foo {
    pub bar: BitString,
    pub baz: u32,
}

impl APerElement for Foo {
    type Result = Self;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut aper::Decoder, constraints: Constraints) -> Result<Self::Result, aper::DecodeError> {
        let bar = BitString::from_aper(decoder , Constraints {
            value: None,
            size: Some(Constraint::new(Some(4), Some(4))),
        });

        let mut baz = u32::from_aper(decoder, UNCONSTRAINED);

        if bar.is_err() {
            return Err(bar.err().unwrap());
        }
        if baz.is_err() {
            return Err(baz.err().unwrap());
        }

        Ok(Foo{
            bar: bar.unwrap(),
            baz: baz.unwrap(),
        })
    }

    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, aper::EncodeError> {
        let mut enc = self.bar.to_aper(Constraints {
            value: None,
            size: Some(Constraint::new(None, Some(4))),
        }).unwrap();

        enc.append(&self.baz.to_aper(UNCONSTRAINED).unwrap());

        Ok(enc)
    }
}
```
