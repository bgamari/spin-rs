#![crate_type = "lib"]
#![warn(missing_docs)]
#![feature(cfg_target_has_atomic)]

//! Synchronization primitives based on spinning

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub use mutex::*;
pub use rw_lock::*;
pub use once::*;

mod mutex;
mod rw_lock;
mod once;
