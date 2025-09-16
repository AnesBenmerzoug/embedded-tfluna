use embedded_hal::i2c::Error as I2CErrorTrait;

use crate::i2c::constants::DEFAULT_SLAVE_ADDRESS;

/// I2C device address
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub(crate) u8);

/// Default device address
impl Default for Address {
    fn default() -> Self {
        Address(DEFAULT_SLAVE_ADDRESS)
    }
}

/// Support custom (integer) addresses
impl From<u8> for Address {
    fn from(a: u8) -> Self {
        Address(a)
    }
}

/// Support conversion of address to integer
impl From<Address> for u8 {
    fn from(a: Address) -> Self {
        a.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2CError: I2CErrorTrait> {
    I2c(I2CError),
    InvalidData,
    InvalidParameter,
    Other,
}

impl<I2CError> From<I2CError> for Error<I2CError>
where
    I2CError: I2CErrorTrait,
{
    fn from(value: I2CError) -> Self {
        Error::I2c(value)
    }
}
