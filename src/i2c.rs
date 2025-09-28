//! Interface for the I2C protocol.
//! 
//! When pin 5 is connected to ground, TF-Luna enters I2C mode,
//! then its pin 2 is used as SDA data and pin 3 is the SCL clock sending data.
//! TF-Luna supports up to 400kps clock speed as slave machine and its default address is 0x10.

mod constants;
mod device;
mod types;

pub use constants::DEFAULT_SLAVE_ADDRESS;
pub use device::TFLuna;
pub use types::{Address, Error};
