#![feature(associated_consts)]
extern crate byteorder;

mod bit_string;
mod octet_string;
pub mod aper;
pub use bit_string::BitString;
pub use octet_string::*;
