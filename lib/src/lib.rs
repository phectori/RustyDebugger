//!
//! Embedded debugger library.
//!

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate crc_all;

pub mod packet;
pub mod protocol;
