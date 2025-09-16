//! This is a platform agnostic Rust driver for the [`TF-Luna`] LiDAR distance sensor,
//! based on the [`embedded-hal`] traits.
//!
//! [`TF-Luna`]: https://en.benewake.com/TFLuna/index.html
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

#![no_std]

pub mod i2c;
pub mod types;
