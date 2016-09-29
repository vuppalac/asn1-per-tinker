extern crate asn1;
use asn1::BitString;
use asn1::aper::{self, Constraint, Constraints};

#[test]
fn get_set() {
    let mut b = BitString::with_len(64);
    assert_eq!(false, b.is_set(0));
    b.set(0, true);
    assert_eq!(true, b.is_set(0));
}

#[test]
fn get_set_non_boundary() {
    let mut b = BitString::with_len(64);
    b.set(9, true);
    assert_eq!(true, b.is_set(9));
}

#[test]
fn decode_padded() {
    let data = b"\x00\xe0\x00";
    let mut d = aper::Decoder::new(data);
    let mut b = d.decode::<BitString>(Constraints {
        value: None,
        size: Some(Constraint::new(None, Some(20))),
    }).unwrap();
    println!("{:?}", b);
    for i in 0..20 {
        if i == 17 || i == 18 || i == 19 {
            assert_eq!(true, b.is_set(i));
        } else {
            assert_eq!(false, b.is_set(i));
        }
    }
}

#[test]
fn decode_padded_small() {
    let data = b"\x0e";
    let mut d = aper::Decoder::new(data);
    let mut b = d.decode::<BitString>(Constraints {
        value: None,
        size: Some(Constraint::new(None, Some(4))),
    }).unwrap();
    println!("{:?}", b);
    for i in 0..4 {
        if i == 1 || i == 2 || i == 3 {
            assert_eq!(true, b.is_set(i));
        } else {
            assert_eq!(false, b.is_set(i));
        }
    }
}

#[test]
fn decode_unpadded() {
    let data = b"\x00\x00\xe0";
    let mut d = aper::Decoder::new(data);
    let mut b = d.decode::<BitString>(Constraints {
        value: None,
        size: Some(Constraint::new(None, Some(24))),
    }).unwrap();
    println!("{:?}", b);
    for i in 0..20 {
        if i == 5 || i == 6 || i == 7 {
            assert_eq!(true, b.is_set(i));
        } else {
            assert_eq!(false, b.is_set(i));
        }
    }
}
