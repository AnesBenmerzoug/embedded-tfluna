#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SerialNumber(pub [u8; 14]);

/// Structure containing distance, signal strength, temperature, and timestamp
/// 
/// 
/// - Distance: In centimeters.
/// - Signal Strength: Signal strength value between 0 and 1000. 
/// - Temperature: In °C with 0.01 precision.
/// - Timestamp: Device ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensorReading {
    pub distance: u16,
    pub signal_strength: u16,
    pub temperature: f32,
    pub timestamp: u16,
}

pub trait TFLunaSync {
    type Error;

    fn combine_buffer_into_word(&self, buffer: &[u8; 2]) -> Result<u16, Self::Error> {
        let value = buffer[0] as u16 + ((buffer[1] as u16) << 8);
        Ok(value)
    }

    /// Restore all settings to factory defaults.
    fn restore_factory_defaults(&mut self) -> Result<(), Self::Error>;

    /// Save current settings to persistent storage.
    fn save_settings(&mut self) -> Result<(), Self::Error>;

    /// Enable the LiDAR sensor for measurement operations.
    fn enable(&mut self) -> Result<(), Self::Error>;

    /// Disable the LiDAR sensor to conserve power.
    fn disable(&mut self) -> Result<(), Self::Error>;

    /// Reboot the device to apply configuration changes or recover from errors.
    fn reboot(&mut self) -> Result<(), Self::Error>;

    /// Get the firmware version of the device.
    fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Self::Error>;

    /// Get the device's unique serial number.
    fn get_serial_number(&mut self) -> Result<SerialNumber, Self::Error>;
    
    /// Perform a complete measurement reading from the sensor.
    ///
    /// Reads distance, signal strength, temperature and timestamp
    fn measure(&mut self) -> Result<SensorReading, Self::Error>;
}
