#![feature(associated_consts)]
extern crate asn1;
use asn1::BitString;
use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};

struct Foo {
    pub foo: BitString,
    pub bar: Vec<u8>,
    pub baz: Vec<BitString>,
}


impl APerElement for Foo {
    type Result = Self;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut aper::Decoder, constraints: Constraints) -> Result<Self::Result, aper::DecodeError> {
        let foo = BitString::from_aper(decoder , Constraints {
            value: None,
            size: Some(Constraint::new(None, Some(4))),
        });

        let mut bar = Vec::<u8>::from_aper(decoder, Constraints {
            value: None,
            size: Some(Constraint::new(None, Some(3))),
        });

        let mut baz = Vec::<BitString>::from_aper(decoder, Constraints {
            // here the "value" constraint is a constraint on the size of each element
            value: Some(Constraint::new(None, Some(4))), 
            // "size" behaves normally 
            size: Some(Constraint::new(None, Some(2))),
        });

        if foo.is_err() || bar.is_err() || baz.is_err() {
            return Err(aper::DecodeError::Dummy);
        }

        Ok(Foo{
            foo: foo.unwrap(),
            bar: bar.unwrap(),
            baz: baz.unwrap(),
        })
    }
}

#[test]
fn decode_foo() {
    let data = b"\x0e\x03\x46\x4f\x4f\x02\x0e\x0e";
    let mut d = aper::Decoder::new(data);
    let f = Foo::from_aper(&mut d, UNCONSTRAINED).unwrap();
    let target_bar = vec![0x46 as u8, 0x4f as u8, 0x4f as u8];

    for i in 0..4 {
        if i == 1 || i == 2 || i == 3 {
            assert_eq!(true, f.foo.is_set(i));
        } else {
            assert_eq!(false, f.foo.is_set(i));
        }
    }

    assert_eq!(f.bar.len(), target_bar.len());
    for i in 0..f.bar.len() {
        assert_eq!(f.bar[i], target_bar[i]);
    }

    assert_eq!(f.baz.len(), 2);
    for i in 0..f.baz.len() {
        for j in 0..4 {
            if j == 1 || j == 2 || j == 3 {
                assert_eq!(true, f.baz[i].is_set(j));
            } else {
                assert_eq!(false, f.baz[i].is_set(j));
            }
        }
    }
}
