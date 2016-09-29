extern crate asn1;
use asn1::aper::{self, Constraint, Constraints};

#[test]
fn decode() {
    let data = b"\x46\x4f\x4f";
    let mut d = aper::Decoder::new(data);
    let mut v = d.decode::<Vec<u8>>(Constraints {
        value: None,
        size: Some(Constraint::new(None, Some(3))),
    }).unwrap();
    assert_eq!(v.len(), data.len());
    for i in 0..v.len() {
        assert_eq!(v[i], data[i]);
    }
    println!("{:?}", v);
}
