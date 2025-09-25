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
    InvalidData(u8),
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


#[derive(Debug, Copy, Clone)]
pub enum Register {
    /// Distance measurement low byte register - centimeters - Read-only
    Distance = 0x00,
    /// Signal strength measurement low byte register - Read-only
    SignalStrength = 0x02,
    /// Temperature measurement low byte register - 0.01°C units - Read-only
    Temperature = 0x04,
    /// Timestamp low byte register - device ticks - Read-only
    Timestamp = 0x06,
    /// Error low byte register - Read-only
    Error = 0x08,
    /// Firmware revision number register - first of three version registers - Read-only
    FirmwareVersion = 0x0A,
    /// Serial number first byte register - 14-byte ASCII code - Read-only
    SerialNumber = 0x10,
    /// Ultra low power mode configuration register - Write-only
    UltraLowPowerMode = 0x1F,
    /// Save settings command register - Write-only
    Save = 0x20,
    /// Shutdown/reboot command register - Write-only
    ShutdownReboot = 0x21,
    /// I2C slave address configuration register - Read/Write
    SlaveAddress = 0x22,
    /// Ranging mode configuration register - Read/Write
    RangingMode = 0x23,
    /// Trigger one-shot measurement command register - Write-only
    Trigger = 0x24,
    /// Enable/disable device register - Read/Write
    Enable = 0x25,
    /// Framerate configuration low byte register - Hz - Read/Write
    Framerate = 0x26,
    /// Power-saving mode configuration register - Read/Write
    PowerSavingMode = 0x28,
    /// Restore factory defaults command register - Write-only
    RestoreFactoryDefaults = 0x29,
    /// Signal strength threshold low byte register - Read/Write
    SignalStrengthThreshold = 0x2A,
    /// Dummy distance low byte register - centimeters - Read/Write
    DummyDistance = 0x2C,
    /// Minimum distance low byte register - centimeters - Read/Write
    MinimumDistance = 0x2E,
    /// Maximum distance low byte register - centimeters - Read/Write
    MaximumDistance = 0x30,
    /// Signature lower byte register - 4-byte ASCII code - Read-only
    Signature = 0x3C,
}
