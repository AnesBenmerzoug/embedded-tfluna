#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]

pub mod i2c;
mod types;

pub use types::{FirmwareVersion, PowerMode, RangingMode, SensorReading, SerialNumber, Signature};
