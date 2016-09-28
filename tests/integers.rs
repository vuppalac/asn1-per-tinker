extern crate asn1;
use asn1::aper;

// XXX: negative value tests
#[test]
fn unconstrained_bounds() {
    let data = b"\x02\x10\x00";
    let mut d = aper::Decoder::new(data);
    assert_eq!(4096, d.decode_int(None, None).unwrap());
}

#[test]
fn constrained_bounds() {
    let data = b"\x00";
    let mut d = aper::Decoder::new(data);
    assert_eq!(Ok(4000), d.decode_int(Some(4000), Some(4255)));
}

#[test]
fn constrained_bounds_unpadded() {
    let data = b"\x06";
    let mut d = aper::Decoder::new(data);
    assert_eq!(Ok(10), d.decode_int(Some(10), Some(12)));
    assert_eq!(Ok(11), d.decode_int(Some(10), Some(12)));
}

#[test]
fn semiconstrainted_bounds() {
    let data = b"\x02\x10\x01";
    let mut d = aper::Decoder::new(data);
    assert_eq!(Ok(4096), d.decode_int(Some(-1), None));
}
