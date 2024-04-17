#![warn(rust_2018_idioms)]

//! Rust binding for DPDK
//!
//! Currently, build.rs cannot configure linker options, thus, a user must set RUSTFLAGS env
//! variable as this library's panic message says.

#[allow(warnings, clippy)]
mod dpdk;
pub use dpdk::*;

#[link(name = "bsd")]
extern "C" {}

#[link(name = "pcap")]
extern "C" {}

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
