#![feature(associated_consts)]
extern crate byteorder;

/// A module for encoding and decoding ASN.1 messages of the Aligned PER flavor.
pub mod aper;

mod bit_string;
mod integer;
mod sequence_of;
mod sequence;
mod bool;
mod extensions;

pub use bit_string::BitString;
pub use bool::*;
pub use extensions::*;
pub use integer::*;
pub use sequence::*;
pub use sequence_of::*;
