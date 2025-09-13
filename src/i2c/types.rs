use crate::i2c::constants::DEFAULT_SLAVE_ADDRESS;

/// I2C device address
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct I2CAddress(pub(crate) u8);

/// Default device address
impl Default for I2CAddress {
    fn default() -> Self {
        I2CAddress(DEFAULT_SLAVE_ADDRESS)
    }
}

/// Support custom (integer) addresses
impl From<u8> for I2CAddress {
    fn from(a: u8) -> Self {
        I2CAddress(a)
    }
}

/// Support conversion of address to integer
impl From<I2CAddress> for u8 {
    fn from(a: I2CAddress) -> Self {
        a.0
    }
}
