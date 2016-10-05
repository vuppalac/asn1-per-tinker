#![feature(associated_consts)]
extern crate asn1;
use asn1::{BitString, ExtensionMarker};
use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};

enum Foo {
    foo { a: BitString, },
    bar { a: Vec<u8>, },
    baz { a: u8, b: u16, },
}

impl APerElement for Foo {
    type Result = Self;
    const TAG: u32 = 0xBEEF;
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut aper::Decoder, constraints: Constraints) -> Result<Self::Result, aper::DecodeError> {
        let is_ext = ExtensionMarker::from_aper(decoder, UNCONSTRAINED);
        if is_ext.is_err() {
            return Err(is_ext.err().unwrap());
        }

        let choice = decoder.decode_int(Some(0), Some(2));
        if choice.is_err() {
            return Err(choice.err().unwrap());
        }

        let c = choice.unwrap();
        println!("{}", c);
        match c {
            0 => {
                let bs = BitString::from_aper(decoder , Constraints {
                    value: None,
                    size: Some(Constraint::new(None, Some(4))),
                });
                if bs.is_err() {
                    Err(bs.err().unwrap())
                } else {
                    Ok(Foo::foo{ a: bs.unwrap(), })
                }
            },
            1 => {
                let mut v = Vec::<u8>::from_aper(decoder, Constraints {
                    value: None,
                    size: Some(Constraint::new(None, Some(3))),
                });
                if v.is_err() {
                    Err(v.err().unwrap())
                } else {
                    Ok(Foo::bar{ a: v.unwrap(), })
                }
            },
            2 => {
                let a = u8::from_aper(decoder, UNCONSTRAINED);
                let b = u16::from_aper(decoder, UNCONSTRAINED);
                if a.is_err() || b.is_err() {
                    Err(a.err().unwrap())
                } else {
                    Ok(Foo::baz{ a: a.unwrap(), b: b.unwrap(), })
                }
            }
            _ => Err(aper::DecodeError::InvalidChoice)
        }
    }
}

#[test]
fn decode_foo() {
    let data = b"\x1c"; // 0001 1100
    let mut d = aper::Decoder::new(data);
    let f = Foo::from_aper(&mut d, UNCONSTRAINED).unwrap();

    match f {
        Foo::foo{a: x} => {
            for i in 0..4 {
                if i == 1 || i == 2 || i == 3 {
                    assert_eq!(true, x.is_set(i));
                } else {
                    assert_eq!(false, x.is_set(i));
                }
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn decode_bar() {
    let data = b"\x01\x03\x46\x4f\x4f";
    let mut d = aper::Decoder::new(data);
    d.read(5); // strip left-padding
    let f = Foo::from_aper(&mut d, UNCONSTRAINED).unwrap();
    let target_bar = vec![0x46 as u8, 0x4f as u8, 0x4f as u8];

    match f {
        Foo::bar{a: x} => {
            assert_eq!(x.len(), target_bar.len());
            for i in 0..x.len() {
                assert_eq!(x[i], target_bar[i]);
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn decode_baz() {
    let data = b"\x02\x88\xf9\x3b";
    let mut d = aper::Decoder::new(data);
    d.read(5); // strip left-padding 
    let f = Foo::from_aper(&mut d, UNCONSTRAINED).unwrap();

    match f {
        Foo::baz{a: x, b: y} => {
            assert_eq!(0x88 as u8, x);
            assert_eq!(0xf93b as u16, y);
        },
        _ => assert!(false),
    }
}
