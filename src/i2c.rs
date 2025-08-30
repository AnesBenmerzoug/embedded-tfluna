pub mod constants;
pub mod errors;

use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::i2c::errors::Error;
use crate::traits::{FirmwareVersion, SensorReading, SerialNumber, TFLunaSync};

#[derive(Debug)]
pub struct TFLuna<I2C: I2c<SevenBitAddress>> {
    i2c: I2C,
    address: SevenBitAddress,
}

impl<I2C> TFLuna<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C, address: SevenBitAddress) -> Result<Self, Error<I2C::Error>> {
        let sensor = Self { i2c, address };
        Ok(sensor)
    }

    /// Write byte to register
    fn write_register(
        &mut self,
        register_address: u8,
        content: u8,
    ) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(self.address, &[register_address, content])?;
        Ok(())
    }

    /// Read the contents of a single register
    fn read_register(&mut self, register_address: u8) -> Result<u8, Error<I2C::Error>> {
        // Send register address first
        self.i2c
            .write(self.address, &[register_address])
            .map_err(Error::I2c)?;
        // Read content of register
        let mut buffer = [0];
        self.i2c
            .read(self.address, &mut buffer)
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
    /// * `Err(Error::I2c(I2CError))` - if there is an I2C error
    fn read_word(&mut self, start_register_address: u8) -> Result<u16, Error<I2C::Error>> {
        let mut buffer = [0; 2];
        buffer[0] = self.read_register(start_register_address)?;
        buffer[1] = self.read_register(start_register_address + 1)?;
        Ok(self.combine_buffer_into_word(&buffer))
    }
}

impl<I2C> TFLunaSync for TFLuna<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = Error<I2C::Error>;

    /// Restore all settings to factory defaults.
    ///
    /// # Notes
    /// Writes 0x01 to the RESTORE_FACTORY_DEFAULTS register (0x29).
    /// This will reset all configurable parameters to their original values.
    fn restore_factory_defaults(&mut self) -> Result<(), Self::Error> {
        self.write_register(
            constants::RESTORE_FACTORY_DEFAULTS_REGISTER_ADDRESS,
            constants::RESTORE_FACTORY_DEFAULTS_COMMAND_VALUE,
        )?;
        Ok(())
    }

    /// Save current settings to persistent storage.
    ///
    /// Writes 0x01 to the SAVE register (0x20) to persist all current
    /// configuration settings to non-volatile memory.
    fn save_settings(&mut self) -> Result<(), Self::Error> {
        self.write_register(constants::SAVE_REGISTER_ADDRESS, constants::SAVE_COMMAND_VALUE)?;
        Ok(())
    }

    /// Set enable bit
    fn enable(&mut self) -> Result<(), Self::Error> {
        self.write_register(constants::ENABLE_REGISTER_ADDRESS, 1)?;
        Ok(())
    }

    /// Unset enable bit
    fn disable(&mut self) -> Result<(), Self::Error> {
        self.write_register(constants::ENABLE_REGISTER_ADDRESS, 0)?;
        Ok(())
    }

    /// Reboots device
    fn reboot(&mut self) -> Result<(), Self::Error> {
        self.write_register(
            constants::SHUTDOWN_REBOOT_REGISTER_ADDRESS,
            constants::REBOOT_COMMAND_VALUE,
        )?;
        Ok(())
    }

    fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Self::Error> {
        let mut buffer = [0; 3];
        for i in 0..=2 {
            buffer[i] =
                self.read_register(constants::FIRMWARE_VERSION_REGISTER_ADDRESS + i as u8)?;
        }
        let version = FirmwareVersion {
            major: buffer[2],
            minor: buffer[1],
            revision: buffer[0],
        };
        Ok(version)
    }

    fn get_serial_number(&mut self) -> Result<crate::traits::SerialNumber, Self::Error> {
        let mut buffer = [0; 14];
        for i in 0..14 {
            buffer[i] = self.read_register(constants::SERIAL_NUMBER_REGISTER_ADDRESS + i as u8)?;
        }
        Ok(SerialNumber(buffer))
    }
    
    fn get_signature(&mut self) -> Result<crate::traits::Signature, Self::Error> {
        todo!()
    }

    /// Set the I2C slave address of the device.
    ///
    /// # Arguments
    /// * `address` - New slave address
    ///
    /// # Returns
    /// * `Ok(())` - if address was set successfully
    /// * `Err(Error::InvalidParameter)` - if address is out of valid range
    /// * `Err(Error::I2c(I2CError))` - if there is an I2C error
    ///
    /// # Notes
    /// * Typically range [0x08, 0x77] for valid addresses
    fn set_i2c_slave_address(&mut self, address: u8) -> Result<(), Error<I2C::Error>> {
        if !(constants::SLAVE_ADDRESS_MINIMUM_VALUE..=constants::SLAVE_ADDRESS_MAXIMUM_VALUE).contains(&address) {
            return Err(Error::InvalidParameter);
        }
        self.write_register(constants::SLAVE_ADDRESS_REGISTER_ADDRESS, address)
    }

    /// Get the current I2C slave address of the device.
    ///
    /// # Returns
    /// * `Ok(u8)` - Current slave address
    /// * `Err(Error::I2c(I2CError))` - if there is an I2C error
    fn get_i2c_slave_address(&mut self) -> Result<u8, Error<I2C::Error>> {
        self.read_register(constants::SLAVE_ADDRESS_REGISTER_ADDRESS)
    }
    
    fn get_power_mode(&mut self) -> Result<crate::traits::PowerMode, Self::Error> {
        todo!()
    }
    
    fn set_power_mode(&mut self, mode: crate::traits::PowerMode) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn wake_from_ultra_low_power(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_ranging_mode(&mut self) -> Result<crate::traits::RangingMode, Self::Error> {
        todo!()
    }
    
    fn set_ranging_mode(&mut self, mode: crate::traits::RangingMode) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_framerate(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }
    
    fn set_framerate(&mut self, framerate: u16) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_signal_strength_threshold(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }
    
    fn set_signal_strength_threshold(&mut self, threshold: u16) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_dummy_distance(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }
    
    fn set_dummy_distance(&mut self, distance: u16) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_minimum_distance(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }
    
    fn set_minimum_distance(&mut self, distance: u16) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_maximum_distance(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }
    
    fn set_maximum_distance(&mut self, distance: u16) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn get_error(&mut self) -> Result<u16, Self::Error> {
        todo!()
    }

    /// Perform a complete measurement reading from the sensor.
    ///
    /// # Returns
    /// * `Ok(SensorReading)` - Structure containing distance, signal strength, temperature, and timestamp
    /// * `Err(Error::I2c(I2CError))` - if there is an I2C error
    ///
    /// # Notes
    /// Reads four 16-bit values from consecutive register pairs:
    /// - Distance: Registers 0x00 (low byte) and 0x01 (high byte) in centimeters
    /// - Signal Strength: Registers 0x02 (low byte) and 0x03 (high byte)
    /// - Temperature: Registers 0x04 (low byte) and 0x05 (high byte) in 0.01°C units
    /// - Timestamp: Registers 0x06 (low byte) and 0x07 (high byte) device ticks
    ///
    /// Temperature is automatically converted from hundredths of degrees Celsius to degrees Celsius.
    fn measure(&mut self) -> Result<crate::traits::SensorReading, Self::Error> {
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
    
    fn trigger_measurement(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    /// Returns vector of i2c transaction expectations for an I2C read operation
    fn read_expectations(register_address: u8, value: u8) -> Vec<I2cTransaction> {
        let expectations = Vec::from([
            I2cTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([register_address]),
            ),
            I2cTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([value])),
        ]);
        expectations
    }

    /// Returns vector of i2c transaction expectations for an I2C write operation
    fn write_expectations(register_address: u8, value: u8) -> Vec<I2cTransaction> {
        let expectations = Vec::from([I2cTransaction::write(
            constants::DEFAULT_SLAVE_ADDRESS,
            Vec::from([register_address, value]),
        )]);
        expectations
    }

    #[test]
    fn test_enable_disable() {
        let mut expectations = Vec::new();
        expectations.extend_from_slice(&write_expectations(constants::ENABLE_REGISTER_ADDRESS, 1));
        expectations.extend_from_slice(&write_expectations(constants::ENABLE_REGISTER_ADDRESS, 0));
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS).unwrap();
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
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS).unwrap();
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
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS).unwrap();
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
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS).unwrap();
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
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS).unwrap();
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
