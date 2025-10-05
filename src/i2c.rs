//! Interface for the I2C protocol.
//!
//! When pin 5 is connected to ground, TF-Luna enters I2C mode.
//! In this mode, pin 2 is used as SDA (Serial Data) and pin 3 as SCL (Serial Clock).
//!
//! TF-Luna supports up to 400kps clock speed as slave machine and its default address is 0x10.
//!
//! | Max transmission rate | 400kbps |
//! |---|---|
//! | Master/Slave mode | Slave |
//! | Default address | 0x10 |
//! | Address range | 0x01~0x7F |

mod constants;
mod types;

#[path = "i2c"]
#[cfg(feature = "async")]
pub mod asynchronous {
    //! Asynchronous I2C interface
    use bisync::asynchronous::*;
    mod device;
    pub use device::*;
}

#[path = "i2c"]
pub mod blocking {
    //! Blocking I2C interface
    use bisync::synchronous::*;
    mod device;
    pub use device::*;
}

pub use blocking::TFLuna;
pub use constants::DEFAULT_SLAVE_ADDRESS;
pub use types::{Address, Error};
