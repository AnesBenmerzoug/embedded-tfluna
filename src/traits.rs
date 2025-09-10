/// Structure containing major, minor, and revision numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SerialNumber(pub [u8; 14]);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Signature(pub [u8; 4]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangingMode {
    Continuous,
    Trigger,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Normal,
    PowerSaving,
    UltraLow,
}

/// Structure containing distance, signal strength, temperature, and timestamp.
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

/// Synchronous interface for TF-Luna LiDAR sensor.
///
/// This trait provides a common interface for both I2C and UART communication
/// with the TF-Luna LiDAR sensor. All methods are synchronous and may block
/// during communication with the device.
pub trait TFLunaSync {
    /// Error type returned by all operations
    ///
    /// Typically wraps communication errors (I2C, UART) and device-specific errors
    type Error;

    /// Combine two bytes from a buffer into a 16-bit word (little-endian).
    ///
    /// # Arguments
    /// * `buffer` - Two-element array containing [low_byte, high_byte]
    ///
    /// # Returns
    /// * `u16` - Combined 16-bit value
    fn combine_buffer_into_word(&self, buffer: &[u8; 2]) -> u16 {
        let value = buffer[0] as u16 + ((buffer[1] as u16) << 8);
        value
    }

    /// Restore all settings to factory defaults.
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Resets all configurable parameters to original values
    /// * May require device reboot to take full effect
    fn restore_factory_defaults(&mut self) -> Result<(), Self::Error>;

    /// Save current settings to persistent storage.
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Persists configuration to non-volatile memory.
    /// * Settings survive power cycles after saving.
    fn save_settings(&mut self) -> Result<(), Self::Error>;

    /// Enable the LiDAR sensor for measurement operations.
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Device must be enabled before taking measurements.
    fn enable(&mut self) -> Result<(), Self::Error>;

    /// Disable the LiDAR sensor to conserve power.
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Reduces power consumption when not in use.
    /// * Measurements cannot be taken while disabled.
    fn disable(&mut self) -> Result<(), Self::Error>;

    /// Reboot the device to apply configuration changes or recover from errors.
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Some configuration changes require reboot to take effect.
    fn reboot(&mut self) -> Result<(), Self::Error>;

    /// Get the firmware version of the device
    ///
    /// # Returns
    /// * `Ok(FirmwareVersion)` - Structure containing major, minor, and revision numbers
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Useful for compatibility checking and debugging.
    fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Self::Error>;

    /// Get the device's unique serial number.
    ///
    /// # Returns
    /// * `Ok(SerialNumber)` - Unique identifier for the device
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * 14-byte ASCII production code.
    /// * Useful for device identification and tracking.
    fn get_serial_number(&mut self) -> Result<SerialNumber, Self::Error>;

    /// Get the device signature.
    ///
    /// # Returns
    /// * `Ok(Signature)` - 4-byte ASCII signature ('L' 'U' 'N' 'A')
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Used for device identification and validation.
    fn get_signature(&mut self) -> Result<Signature, Self::Error>;

    /// Get the current I2C slave address of the device.
    ///
    /// # Returns
    /// * `Ok(u8)` - Current slave address
    /// * `Err(Self::Error)` - if there was an error
    fn get_i2c_slave_address(&mut self) -> Result<u8, Self::Error>;

    /// Set the I2C slave address of the device.
    ///
    /// # Arguments
    /// * `address` - New slave address
    ///
    /// # Returns
    /// * `Ok(())` - if address was set successfully
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Typically range [0x08, 0x77] for valid addresses
    fn set_i2c_slave_address(&mut self, address: u8) -> Result<(), Self::Error>;

    /// Get the current power mode of the device
    ///
    /// # Returns
    /// * `Ok(PowerMode)` - Current power mode
    /// * `Err(Self::Error)` - if there was an error
    fn get_power_mode(&mut self) -> Result<PowerMode, Self::Error>;

    /// Set the power mode of the device
    ///
    /// # Arguments
    /// * `mode` - [`PowerMode::Normal`], [`PowerMode::PowerSaving`], or [`PowerMode::UltraLow`]
    ///
    /// # Notes
    /// * Power saving modes reduce consumption at performance cost
    /// * Ultra-low power mode has special wake-up requirements
    fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Self::Error>;

    /// Wake up the device from ultra-low power mode for configuration
    ///
    /// # Notes
    /// * Required before sending commands when in ultra-low power mode
    /// * Includes necessary delay after wake-up
    /// * Reading any register typically wakes the device
    fn wake_from_ultra_low_power(&mut self) -> Result<(), Self::Error>;

    /// Get the current ranging mode of the device
    ///
    /// # Returns
    /// * `Ok(RangingMode)` - Current ranging mode
    /// * `Err(Self::Error)` - if there was an error
    fn get_ranging_mode(&mut self) -> Result<RangingMode, Self::Error>;

    /// Set the ranging mode of the device
    ///
    /// # Arguments
    /// * `mode` - [`RangingMode::Continuous`] or [`RangingMode::Trigger`]
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * In trigger mode, use [`trigger_measurement()`] to initiate measurements
    fn set_ranging_mode(&mut self, mode: RangingMode) -> Result<(), Self::Error>;

    /// Get the current measurement framerate in Hz
    ///
    /// # Returns
    /// * `Ok(u16)` - Current framerate in Hz
    /// * `Err(Self::Error)` - if there was an error
    fn get_framerate(&mut self) -> Result<u16, Self::Error>;

    /// Set the measurement framerate in Hz
    ///
    /// # Arguments
    /// * `framerate` - Desired framerate in Hz
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Higher framerates consume more power
    /// * Lower framerates improve power efficiency
    fn set_framerate(&mut self, framerate: u16) -> Result<(), Self::Error>;

    /// Get the current signal strength threshold
    ///
    /// # Returns
    /// * `Ok(u16)` - Current signal strength threshold value
    /// * `Err(Self::Error)` - if there was an error
    fn get_signal_strength_threshold(&mut self) -> Result<u16, Self::Error>;

    /// Set the signal strength threshold for valid measurements
    ///
    /// # Arguments
    /// * `value` - Minimum signal strength for valid measurements
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Measurements below threshold may be considered invalid
    /// * Helps filter low-quality or noisy readings
    fn set_signal_strength_threshold(&mut self, value: u16) -> Result<(), Self::Error>;

    /// Get the current dummy distance value
    ///
    /// # Returns
    /// * `Ok(u16)` - Current dummy distance value
    /// * `Err(Self::Error)` - if there was an error
    fn get_dummy_distance(&mut self) -> Result<u16, Self::Error>;

    /// Set the dummy distance value for testing purposes
    ///
    /// # Arguments
    /// * `value` - Dummy distance value
    /// 
    /// # Returns
    /// * `Err(Self::Error)` - if there was an error
    fn set_dummy_distance(&mut self, value: u16) -> Result<(), Self::Error>;

    /// Get the current minimum distance setting
    ///
    /// # Returns
    /// * `Ok(u16)` - Current minimum distance
    /// * `Err(Self::Error)` - if there was an error
    fn get_minimum_distance(&mut self) -> Result<u16, Self::Error>;

    /// Set the minimum valid distance measurement
    ///
    /// # Arguments
    /// * `value` - Minimum distance in appropriate units
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Measurements below this distance may be filtered
    /// * Units depend on interface (mm for I2C, cm for UART typically)
    fn set_minimum_distance(&mut self, value: u16) -> Result<(), Self::Error>;

    /// Get the current maximum distance setting
    ///
    /// # Returns
    /// * `Ok(u16)` - Current maximum distance
    /// * `Err(Self::Error)` - if there was an error
    fn get_maximum_distance(&mut self) -> Result<u16, Self::Error>;

    /// Set the maximum valid distance measurement
    ///
    /// # Arguments
    /// * `value` - Maximum distance in appropriate units
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Measurements above this distance may be filtered
    /// * Units depend on interface (mm for I2C, cm for UART typically)
    fn set_maximum_distance(&mut self, value: u16) -> Result<(), Self::Error>;

    /// Get the error code from the device
    ///
    /// # Returns
    /// * `Ok(u16)` - Error code value
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Useful for debugging communication or device issues
    /// * Consult device manual for error code meanings
    fn get_error(&mut self) -> Result<u16, Self::Error>;

    /// Perform a complete measurement reading from the sensor.
    ///
    /// Reads distance, signal strength, temperature and timestamp
    fn measure(&mut self) -> Result<SensorReading, Self::Error>;

    /// Trigger a single measurement (only effective in trigger mode).
    ///
    /// # Returns
    /// * `Ok(())` - if trigger was set successfully
    /// * `Err(Self::Error)` - if there was an error
    ///
    /// # Notes
    /// * Only works when device is in [`RangingMode::Trigger`].
    /// * Initiates immediate measurement in trigger mode.
    fn trigger_measurement(&mut self) -> Result<(), Self::Error>;
}
