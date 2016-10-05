mod decoder;
pub use self::decoder::{Decoder, DecodeError};

/// An interval that desribes the limits on some value.
/// To indicate something is unbounded, set `min` and `max` to `None`.
#[derive(Debug, Copy, Clone)]
pub struct Constraint {
    min: Option<i64>,
    max: Option<i64>,
}

impl Constraint {
    /// Construct a new `Constraint`.
    pub fn new(min: Option<i64>, max: Option<i64>) -> Constraint {
        Constraint {
            min: min,
            max: max,
        }
    }

    /// Get the lower bound.
    pub fn min(&self) -> Option<i64> {
        self.min
    }

    /// Get the upper bound.
    pub fn max(&self) -> Option<i64> {
        self.max
    }
}

/// A pair of `Constraint`s that describes the constraints on the value (if applicable) and encoded size of a type.
/// A value is considered unconstrained if `value` and `size` are both set to `None`.
#[derive(Debug, Copy, Clone)]
pub struct Constraints {
    pub value: Option<Constraint>,
    pub size: Option<Constraint>,
}

pub const UNCONSTRAINED: Constraints = Constraints {
    value: None,
    size: None,
};

/// Trait for Aligned PER encoding/decoding.
///
/// # Examples
///
/// Consider a simple ASN.1 Sequence `foo` made up of a `BitString` and a 32-bit non-negative integer. 
///
/// ```
/// foo ::= SEQUENCE {
///     bar BIT STRING(SIZE(4)
///     baz INTEGER(0..4294967295)
/// }
/// ```
///
/// The corresponding struct and `APerElement` implementation are shown below.
///
/// ```
/// #![feature(associated_consts)]
/// extern crate asn1;
/// use asn1::BitString;
/// use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};
///
/// struct foo {
///     pub bar: BitString,
///     pub baz: u32,
/// }
///
/// impl APerElement for Foo {
///    type Result = Self;
///    const TAG: u32 = 0xBEEF;
///    const CONSTRAINTS: Constraints = UNCONSTRAINED;
///    fn from_aper(decoder: &mut aper::Decoder, constraints: Constraints) -> Result<Self::Result, aper::DecodeError> {
///        let bar = BitString::from_aper(decoder , Constraints {
///            value: None,
///            size: Some(Constraint::new(Some(4), Some(4))),
///        });
///
///        let mut baz = u32::from_aper(decoder, UNCONSTRAINED);
///
///        if bar.is_err() || baz.is_err() {
///            return Err(aper::DecodeError::Dummy);
///        }
///
///        Ok(Foo{
///            bar: bar.unwrap(),
///            baz: baz.unwrap(),
///        })
///    }
/// }
/// ```
///
/// Now let's consider an enum that corresponds to the ASN.1 Choice type below. (Note the extension marker)
///
/// ```
/// Foo ::= SEQUENCE {
///     a BIT STRING(SIZE(4))
/// }
///
/// Bar ::= SEQUENCE {
///     a OCTET STRING
/// }
///
/// Baz ::= SEQUENCE {
///     a INTEGER(0..255)
///     b INTEGER(0..65535)
/// }
///
/// MyMsg ::= CHOICE {
///     foo Foo
///     bar Bar
///     baz Baz
///     ...
/// }
/// ```
///
/// The corresponding enum and `APerElement` implementation would look like this.
///
/// ```
/// #![feature(associated_consts)]
/// extern crate asn1;
/// use asn1::BitString;
/// use asn1::aper::{self, APerElement, Constraint, Constraints, UNCONSTRAINED};
///
/// enum MyMsg {
///     foo { a: BitString, },
///     bar { a: Vec<u8>, },
///     baz { a: u8, b: u16, },
/// }
/// 
/// impl APerElement for MyMsg {
///     type Result = Self;
///     const TAG: u32 = 0xBEEF;
///     const CONSTRAINTS: Constraints = UNCONSTRAINED;
///     fn from_aper(decoder: &mut aper::Decoder, constraints: Constraints) -> Result<Self::Result, aper::DecodeError> {
///         let is_ext = ExtensionMarker::from_aper(decoder, UNCONSTRAINED);
///         if is_ext.is_err() {
///             return Err(aper::DecodeError::Dummy);
///         }
/// 
///         let choice = decoder.decode_int(Some(0), Some(2));
///         if choice.is_err() {
///             return Err(aper::DecodeError::Dummy);
///         }
/// 
///         match c.unwrap() {
///             0 => {
///                 let bs = BitString::from_aper(decoder , Constraints {
///                     value: None,
///                     size: Some(Constraint::new(None, Some(4))),
///                 });
///                 if bs.is_err() {
///                     Err(aper::DecodeError::Dummy)
///                 } else {
///                     Ok(MyMsg::foo{ a: bs.unwrap(), })
///                 }
///             },
///             1 => {
///                 let mut v = Vec::<u8>::from_aper(decoder, Constraints {
///                     value: None,
///                     size: Some(Constraint::new(None, Some(3))),
///                 });
///                 if v.is_err() {
///                     Err(aper::DecodeError::Dummy)
///                 } else {
///                     Ok(MyMsg::bar{ a: v.unwrap(), })
///                 }
///             },
///             2 => {
///                 let a = u8::from_aper(decoder, UNCONSTRAINED);
///                 let b = u16::from_aper(decoder, UNCONSTRAINED);
///                 if a.is_err() || b.is_err() {
///                     Err(aper::DecodeError::Dummy)
///                 } else {
///                     Ok(MyMsg::baz{ a: a.unwrap(), b: b.unwrap(), })
///                 }
///             }
///             _ => Err(aper::DecodeError::Dummy)
///         }
///     }
/// }
/// ```
pub trait APerElement {
    /// The type to be returned by `from_aper`, usually `Self`
    type Result;

    /// A tag for determining canonical-PER ordering. Not currently used.
    const TAG: u32;

    /// PER-visible Constraints
    const CONSTRAINTS: Constraints;

    /// Constructor for the `Result` type given an aligned PER encoding.
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self::Result, decoder::DecodeError>;
}
