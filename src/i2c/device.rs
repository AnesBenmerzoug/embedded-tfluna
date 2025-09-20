use crate::i2c::constants;
use crate::i2c::types::{Address, Error};

use crate::types::{
    FirmwareVersion, PowerMode, RangingMode, SensorReading, SerialNumber, Signature,
};

use embedded_hal::i2c::{Error as I2CError, ErrorKind};
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c as I2cTrait, SevenBitAddress},
};

// TF-Luna controller
#[derive(Debug)]
pub struct TFLuna<I2C: I2cTrait<SevenBitAddress>, D: DelayNs> {
    /// Concrete I2C device implementation.
    i2c: I2C,
    /// I2C device address
    address: Address,
    delay: D,
}

impl<I2C, D> TFLuna<I2C, D>
where
    I2C: I2cTrait<SevenBitAddress>,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: Address, delay: D) -> Result<Self, Error<I2C::Error>> {
        let sensor = Self {
            i2c,
            address,
            delay,
        };
        Ok(sensor)
    }

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

    /// Write byte to register
    fn write_byte(&mut self, register_address: u8, content: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(self.address.into(), &[register_address, content])?;
        Ok(())
    }

    /// Read the contents of a single register
    fn read_byte(&mut self, register_address: u8) -> Result<u8, Error<I2C::Error>> {
        let mut buffer = [0];
        self.i2c
            .write_read(self.address.into(), &[register_address], &mut buffer)
            .map_err(Error::I2c)?;
        Ok(buffer[0])
    }

    /// Read word (two bytes) from two consecutive registers
    ///
    /// # Arguments
    /// * `start_register_address` - Address of first register. Second's register's address will be `start_register_address + 1`
    ///
    /// # Returns
    /// * Ok(u16) - Read and combined value from consecutive registers
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    fn read_word(&mut self, start_register_address: u8) -> Result<u16, Error<I2C::Error>> {
        let mut buffer = [0; 2];
        buffer[0] = self.read_byte(start_register_address)?;
        buffer[1] = self.read_byte(start_register_address + 1)?;
        Ok(self.combine_buffer_into_word(&buffer))
    }

    /// Write word (two bytes) into two consecutive registers.
    ///
    /// # Arguments
    /// * `start_register_address` - Address of first register. Second's register's address will be `start_register_address + 1`
    ///
    /// # Returns
    /// * Ok(()) - Write and combined value from consecutive registers
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// - The value is stored as a 16-bit value across two registers in little-endian order.
    /// - Low byte is written in register at start address.
    /// - High byte is written in register at start address + 1.
    fn write_word(
        &mut self,
        start_register_address: u8,
        value: u16,
    ) -> Result<(), Error<I2C::Error>> {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;
        self.write_byte(start_register_address, low_byte)?;
        self.write_byte(start_register_address + 1, high_byte)?;
        Ok(())
    }

    /// Restore all settings to factory defaults.
    ///
    /// # Notes
    /// Writes 0x01 to the RESTORE_FACTORY_DEFAULTS register (0x29).
    /// This will reset all configurable parameters to their original values.
    pub fn restore_factory_defaults(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::RESTORE_FACTORY_DEFAULTS_REGISTER_ADDRESS,
            constants::RESTORE_FACTORY_DEFAULTS_COMMAND_VALUE,
        )?;
        Ok(())
    }

    /// Save current settings to persistent storage.
    ///
    /// Writes 0x01 to the SAVE register (0x20) to persist all current
    /// configuration settings to non-volatile memory.
    pub fn save_settings(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::SAVE_REGISTER_ADDRESS,
            constants::SAVE_COMMAND_VALUE,
        )?;
        Ok(())
    }

    /// Set enable bit
    pub fn enable(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::ENABLE_REGISTER_ADDRESS,
            constants::ENABLE_COMMAND_VALUE,
        )?;
        Ok(())
    }

    /// Unset enable bit
    pub fn disable(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::ENABLE_REGISTER_ADDRESS,
            constants::DISABLE_COMMAND_VALUE,
        )?;
        Ok(())
    }

    /// Reboots device
    pub fn reboot(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::SHUTDOWN_REBOOT_REGISTER_ADDRESS,
            constants::REBOOT_COMMAND_VALUE,
        )?;
        // Wait for a second for the device to be ready again
        self.delay.delay_ms(1000);
        Ok(())
    }

    pub fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Error<I2C::Error>> {
        let mut buffer = [0; 3];
        for i in 0..=2 {
            buffer[i] = self.read_byte(constants::FIRMWARE_VERSION_REGISTER_ADDRESS + i as u8)?;
        }
        let version = FirmwareVersion {
            major: buffer[2],
            minor: buffer[1],
            revision: buffer[0],
        };
        Ok(version)
    }

    pub fn get_serial_number(&mut self) -> Result<SerialNumber, Error<I2C::Error>> {
        let mut buffer = [0; 14];
        for i in 0..14 {
            buffer[i] = self.read_byte(constants::SERIAL_NUMBER_REGISTER_ADDRESS + i as u8)?;
        }
        Ok(SerialNumber(buffer))
    }

    /// Get the device signature ('L' 'U' 'N' 'A').
    ///
    /// # Returns
    /// * `Ok([u8; 4])` - 4-byte ASCII signature
    ///
    /// # Notes
    /// Reads 4 consecutive registers starting at 0x3C (0x3C through 0x3F).
    pub fn get_signature(&mut self) -> Result<Signature, Error<I2C::Error>> {
        let mut buffer = [0; 4];
        for i in 0..4 {
            buffer[i] = self.read_byte(constants::SIGNATURE_REGISTER_ADDRESS + i as u8)?;
        }
        Ok(Signature(buffer))
    }

    /// Set the I2C slave address of the device.
    ///
    /// # Arguments
    /// * `address` - New slave address
    ///
    /// # Returns
    /// * `Ok(())` - if address was set successfully
    /// * `Err(Error::InvalidParameter)` - if address is out of valid range
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// * Typically range [0x08, 0x77] for valid addresses
    pub fn set_slave_address(&mut self, address: u8) -> Result<(), Error<I2C::Error>> {
        if !(constants::SLAVE_ADDRESS_MINIMUM_VALUE..=constants::SLAVE_ADDRESS_MAXIMUM_VALUE)
            .contains(&address)
        {
            return Err(Error::InvalidParameter);
        }
        self.write_byte(constants::SLAVE_ADDRESS_REGISTER_ADDRESS, address)
    }

    /// Get the current I2C slave address of the device.
    ///
    /// # Returns
    /// * `Ok(u8)` - Current slave address
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    pub fn get_slave_address(&mut self) -> Result<u8, Error<I2C::Error>> {
        self.read_byte(constants::SLAVE_ADDRESS_REGISTER_ADDRESS)
    }

    /// Get the current power mode of the device.
    ///
    /// # Returns
    /// * `Ok(PowerMode)` - Current power mode
    /// * `Err(Error::InvalidState)` if registers contain invalid values
    ///
    /// # Notes
    /// Reading registers may wake up the device from ultra-low power mode.
    /// Avoid frequent calls when ultra-low power mode is expected.
    pub fn get_power_mode(&mut self) -> Result<PowerMode, Error<I2C::Error>> {
        let power_mode_value = self.read_byte(constants::POWER_MODE_REGISTER_ADDRESS);

        match power_mode_value {
            Ok(0x00) => Ok(PowerMode::Normal),
            Ok(0x01) => Ok(PowerMode::PowerSaving),
            Err(Error::<I2C::Error>::I2c(e)) => {
                // Check if the I2C error is a NoAcknowledge error
                if let ErrorKind::NoAcknowledge(_) = e.kind() {
                    Ok(PowerMode::UltraLow)
                } else {
                    // Return the original I2C error for other error kinds
                    Err(Error::I2c(e))
                }
            }
            Ok(_) => Err(Error::InvalidData),
            Err(e) => Err(e), // For non-I2C errors
        }
    }

    /// Set the power mode of the device.
    ///
    /// # Arguments
    /// * `mode` - [`PowerMode::Normal`], [`PowerMode::PowerSaving`], or [`PowerMode::UltraLow`]
    ///
    /// # Notes
    /// - Power saving modes may reduce power consumption at the cost of performance
    /// - Ultra-low power mode requires special 3-byte sequence and has restrictions
    /// - Do not send setup commands while in ultra-low power mode
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error<I2C::Error>> {
        match mode {
            PowerMode::Normal => {
                self.disable_ultra_low_power_mode()?;
                self.write_byte(
                    constants::POWER_MODE_REGISTER_ADDRESS,
                    constants::NORMAL_POWER_MODE_COMMAND_VALUE
                )?;
                self.delay.delay_ms(100);
            }
            PowerMode::PowerSaving => {
                self.disable_ultra_low_power_mode()?;
                self.write_byte(
                    constants::POWER_MODE_REGISTER_ADDRESS,
                    constants::POWER_SAVING_POWER_MODE_COMMAND_VALUE,
                )?;
                self.delay.delay_ms(100);
            }
            PowerMode::UltraLow => {
                self.enable_ultra_low_power_mode()?;
            }
        }
        Ok(())
    }

    // Set ultra-low power mode, save settings and reboot
    pub fn enable_ultra_low_power_mode(&mut self) -> Result<(), Error<I2C::Error>>{
        self.write_byte(
            constants::ULTRA_LOW_POWER_POWER_MODE_REGISTER_ADDRESS,
            constants::ULTRA_LOWER_POWER_MODE_COMMAND_VALUE,
        )?;
        self.save_settings()?;
        self.reboot()?;
        Ok(())
    }

    pub fn disable_ultra_low_power_mode(&mut self) -> Result<(), Error<I2C::Error>>{
        self.wake_from_ultra_low_power().unwrap();
        self.write_byte(
            constants::ULTRA_LOW_POWER_POWER_MODE_REGISTER_ADDRESS,
            constants::NORMAL_POWER_MODE_COMMAND_VALUE,
        )?;
        self.save_settings()?;
        self.reboot()?;
        Ok(())
    }

    pub fn wake_from_ultra_low_power(&mut self) -> Result<(), Error<I2C::Error>> {
        // Wake up by reading any register
        match self.read_byte(constants::DISTANCE_REGISTER_ADDRESS) {
            Err(Error::<I2C::Error>::I2c(e)) => {
                // Check if the I2C error is a NoAcknowledge error
                if let ErrorKind::NoAcknowledge(_) = e.kind() {
                    ()
                } else {
                    // Return the original I2C error for other error kinds
                    return Err(Error::I2c(e))
                }
            }
            Ok(_) => return Ok(()),
            _ => return Err(Error::InvalidData),
        }
        // Wait at least 6ms after awakening as per manual
        self.delay.delay_ms(6);
        Ok(())
    }

    /// Get the current ranging mode of the device.
    ///
    /// # Returns
    /// * `Ok(RangingMode)` - Current ranging mode
    /// * `Err(Error::InvalidData)` if register contains invalid value
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    pub fn get_ranging_mode(&mut self) -> Result<RangingMode, Error<I2C::Error>> {
        let mode = self.read_byte(constants::RANGING_MODE_REGISTER_ADDRESS)?;
        match mode {
            constants::RANGING_MODE_CONTINUOUS_COMMAND_VALUE => Ok(RangingMode::Continuous),
            constants::RANGING_MODE_TRIGGER_COMMAND_VALUE => Ok(RangingMode::Trigger),
            _ => Err(Error::<I2C::Error>::InvalidData),
        }
    }

    /// Set the ranging mode of the device.
    ///
    /// # Arguments
    /// * `mode` - [`RangingMode::Continuous`] or [`RangingMode::Trigger`]
    ///
    /// # Returns
    /// * `Ok(())` - if ranging mode was set successfully.
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// In trigger mode, use [`trigger_measurement()`] to initiate measurements.
    pub fn set_ranging_mode(&mut self, mode: RangingMode) -> Result<(), Error<I2C::Error>> {
        let mode_value = match mode {
            RangingMode::Continuous => constants::RANGING_MODE_CONTINUOUS_COMMAND_VALUE,
            RangingMode::Trigger => constants::RANGING_MODE_TRIGGER_COMMAND_VALUE,
        };
        self.write_byte(constants::RANGING_MODE_REGISTER_ADDRESS, mode_value)?;
        Ok(())
    }

    /// Get the current measurement framerate in Hz.
    ///
    /// # Returns
    /// * `Ok(u16)` - Current framerate in Hz
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    pub fn get_framerate(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::FRAMERATE_REGISTER_ADDRESS)
    }

    /// Set the measurement framerate in Hz.
    ///
    /// # Arguments
    /// * `framerate` - Desired framerate in Hz (only valid values)
    ///
    /// # Returns
    /// * `Ok(())` - if framerate was set successfully.
    /// * `Err(Error::InvalidParameter)` - if framerate is invalid.
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    ///
    /// # Notes
    /// - The framerate is stored as a 16-bit value across two registers.
    /// - Only factors of 500Hz / n, where n in [2, 3, ...], are allowed.
    pub fn set_framerate(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        match value {
            x if x < 500 && (500 % x) == 0 => {
                self.write_word(constants::FRAMERATE_REGISTER_ADDRESS, value)
            }
            _ => Err(Error::<I2C::Error>::InvalidParameter),
        }
    }

    /// Get the current signal strength threshold.
    ///
    /// # Returns
    /// * `Ok(u16)` - Current signal strength threshold value
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// When Signal Strength < Signal Strength Threshold * 10,
    /// then the returned distance is the dummy distance instead of the actual distance
    pub fn get_signal_strength_threshold(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::SIGNAL_STRENGTH_THRESHOLD_REGISTER_ADDRESS)
    }

    /// Set the signal strength threshold for valid measurements.
    ///
    /// # Arguments
    /// * `value` - Minimum signal strength value for valid measurements.
    ///
    /// # Returns
    /// * `Ok(())` - if signal strength threshold was set successfully.
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    ///
    /// # Notes
    /// When Signal Strength < Signal Strength Threshold * 10,
    /// then the returned distance is the dummy distance instead of the actual distance
    pub fn set_signal_strength_threshold(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(constants::SIGNAL_STRENGTH_THRESHOLD_REGISTER_ADDRESS, value)
    }

    /// Get the current dummy distance value
    ///
    /// # Returns
    /// * `Ok(u16)` - Current dummy distance value
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    pub fn get_dummy_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::DUMMY_DISTANCE_REGISTER_ADDRESS)
    }

    /// Set the dummy distance value
    ///
    /// # Arguments
    /// * `distance` - Dummy distance value for testing
    ///
    /// # Returns
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    pub fn set_dummy_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(constants::DUMMY_DISTANCE_REGISTER_ADDRESS, value)
    }

    /// Get the current maximum distance setting
    ///
    /// # Returns
    /// * `Ok(u16)` - Current maximum distance
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    pub fn get_minimum_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::MINIMUM_DISTANCE_REGISTER_ADDRESS)
    }

    /// Set the minimum valid distance measurement
    ///
    /// # Arguments
    /// * `value` - Minimum distance in appropriate units
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    ///
    /// # Notes
    /// * Measurements below this distance may be filtered
    pub fn set_minimum_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(constants::MINIMUM_DISTANCE_REGISTER_ADDRESS, value)
    }

    /// Get the current maximum distance setting
    ///
    /// # Returns
    /// * `Ok(u16)` - Current maximum distance
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    pub fn get_maximum_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::MAXIMUM_DISTANCE_REGISTER_ADDRESS)
    }

    /// Set the maximum valid distance measurement
    ///
    /// # Arguments
    /// * `value` - Maximum distance in appropriate units
    ///
    /// # Returns
    /// * `Ok(())` - if operation was successful
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    ///
    /// # Notes
    /// * Measurements above this distance may be filtered
    /// * Units depend on interface (mm for I2C, cm for UART typically)
    pub fn set_maximum_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(constants::MAXIMUM_DISTANCE_REGISTER_ADDRESS, value)
    }

    /// Get the error code from the device
    ///
    /// # Returns
    /// * `Ok(u16)` - Error code value
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error.
    pub fn get_error(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(constants::ERROR_REGISTER_ADDRESS)
    }

    /// Perform a complete measurement reading from the sensor.
    ///
    /// # Returns
    /// * `Ok(SensorReading)` - Structure containing distance, signal strength, temperature, and timestamp
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// Reads four 16-bit values from consecutive register pairs:
    /// - Distance: Registers 0x00 (low byte) and 0x01 (high byte) in centimeters
    /// - Signal Strength: Registers 0x02 (low byte) and 0x03 (high byte)
    /// - Temperature: Registers 0x04 (low byte) and 0x05 (high byte) in 0.01°C units
    /// - Timestamp: Registers 0x06 (low byte) and 0x07 (high byte) device ticks
    ///
    /// Temperature is automatically converted from hundredths of degrees Celsius to degrees Celsius.
    pub fn measure(&mut self) -> Result<SensorReading, Error<I2C::Error>> {
        let distance = self.read_word(constants::DISTANCE_REGISTER_ADDRESS)?;
        let signal_strength = self.read_word(constants::SIGNAL_STRENGTH_REGISTER_ADDRESS)?;
        let temperature = self.read_word(constants::TEMPERATURE_REGISTER_ADDRESS)? as f32 / 100.0;
        let timestamp = self.read_word(constants::TIMESTAMP_REGISTER_ADDRESS)?;
        Ok(SensorReading {
            distance,
            signal_strength,
            temperature,
            timestamp,
        })
    }

    pub fn read_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        let distance = self.read_word(constants::DISTANCE_REGISTER_ADDRESS)?;
        Ok(distance)
    }

    /// Trigger a single measurement (only effective in trigger mode).
    ///
    /// # Returns
    /// * `Ok(())` - if trigger was set successfully
    /// * `Err(Error::I2c(I2CError))` - if there was an I2C error
    ///
    /// # Notes
    /// * Only works when device is in [`RangingMode::Trigger`].
    /// * Initiates immediate measurement in trigger mode.
    pub fn trigger_measurement(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            constants::TRIGGER_REGISTER_ADDRESS,
            constants::TRIGGER_COMMAND_VALUE,
        )?;
        Ok(())
    }
}

#[cfg(all(test, not(target_arch = "riscv32imc-unknown-none-elf")))]
mod test {
    extern crate std;
    use std::vec::Vec;

    use super::*;
    use embedded_hal_mock::eh1::delay::StdSleep as Delay;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cTraitMock, Transaction as I2cTraitTransaction};

    /// Returns vector of i2c transaction expectations for an I2C read operation
    fn read_expectations(register_address: u8, value: u8) -> Vec<I2cTraitTransaction> {
        let expectations = Vec::from([
            I2cTraitTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([register_address]),
            ),
            I2cTraitTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([value])),
        ]);
        expectations
    }

    /// Returns vector of i2c transaction expectations for an I2C write operation
    fn write_expectations(register_address: u8, value: u8) -> Vec<I2cTraitTransaction> {
        let expectations = Vec::from([I2cTraitTransaction::write(
            constants::DEFAULT_SLAVE_ADDRESS,
            Vec::from([register_address, value]),
        )]);
        expectations
    }

    fn setup(i2c: &mut I2cTraitMock) -> TFLuna<&mut I2cTraitMock, Delay> {
        TFLuna::new(i2c, constants::DEFAULT_SLAVE_ADDRESS, Delay {}).unwrap()
    }

    #[test]
    fn test_enable_disable() {
        let mut expectations = Vec::new();
        expectations.extend_from_slice(&write_expectations(constants::ENABLE_REGISTER_ADDRESS, 1));
        expectations.extend_from_slice(&write_expectations(constants::ENABLE_REGISTER_ADDRESS, 0));
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        assert!(device.enable().is_ok());
        assert!(device.disable().is_ok());
        i2c.done();
    }

    #[test]
    fn test_reboot() {
        let expectations = write_expectations(
            constants::SHUTDOWN_REBOOT_REGISTER_ADDRESS,
            constants::REBOOT_COMMAND_VALUE,
        );
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        assert!(device.reboot().is_ok());
        i2c.done();
    }

    #[test]
    fn test_get_firmware_version() {
        let mut expectations = Vec::new();
        for i in 0..3 {
            expectations.extend_from_slice(&read_expectations(
                constants::FIRMWARE_VERSION_REGISTER_ADDRESS + i,
                i,
            ));
        }
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        let firmware_version = device.get_firmware_version();
        assert!(firmware_version.is_ok(), "{:?}", firmware_version);
        let expected_firmware_version = FirmwareVersion {
            major: 2,
            minor: 1,
            revision: 0,
        };
        assert_eq!(
            firmware_version.unwrap(),
            expected_firmware_version,
            "{:?} is different from {:?}",
            firmware_version,
            expected_firmware_version
        );
        i2c.done();
    }

    #[test]
    fn test_get_serial_number() {
        let mut expectations = Vec::new();
        for i in 0..14 {
            expectations.extend_from_slice(&read_expectations(
                constants::SERIAL_NUMBER_REGISTER_ADDRESS + i,
                i,
            ));
        }
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        let serial_number = device.get_serial_number();
        assert!(serial_number.is_ok(), "{:?}", serial_number);
        let expected_serial_number =
            SerialNumber((0..14).into_iter().collect::<Vec<_>>().try_into().unwrap());
        assert_eq!(
            serial_number.unwrap(),
            expected_serial_number,
            "{:?} is different from {:?}",
            serial_number,
            expected_serial_number
        );
        i2c.done();
    }

    #[test]
    fn test_measure() {
        let mut expectations = Vec::new();
        // Distance
        for (i, value) in (0..2).zip([0x0A, 0]) {
            expectations.extend_from_slice(&read_expectations(
                constants::DISTANCE_REGISTER_ADDRESS + i,
                value,
            ));
        }
        // Signal Strength
        for (i, value) in (0..2).zip([0x64, 0]) {
            expectations.extend_from_slice(&read_expectations(
                constants::SIGNAL_STRENGTH_REGISTER_ADDRESS + i,
                value,
            ));
        }
        // Temperature
        for (i, value) in (0..2).zip([0xB2, 0x0C]) {
            expectations.extend_from_slice(&read_expectations(
                constants::TEMPERATURE_REGISTER_ADDRESS + i,
                value,
            ));
        }
        // Timestamp
        for (i, value) in (0..2).zip([0, 0]) {
            expectations.extend_from_slice(&read_expectations(
                constants::TIMESTAMP_REGISTER_ADDRESS + i,
                value,
            ));
        }
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        let sensor_reading = device.measure();
        assert!(sensor_reading.is_ok(), "{:?}", sensor_reading);
        let expected_sensor_reading = SensorReading {
            distance: 10,
            signal_strength: 100,
            temperature: 32.5,
            timestamp: 0,
        };
        assert_eq!(
            sensor_reading.unwrap(),
            expected_sensor_reading,
            "{:?} is different from {:?}",
            sensor_reading,
            expected_sensor_reading
        );
        i2c.done();
    }
}
