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
        let sensor = Self {
            i2c,
            address,
        };
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

    /// Register word (two bytes) from consecutive registers
    fn read_word(&mut self, start_register_address: u8) -> Result<u16, Error<I2C::Error>> {
        let mut buffer = [0; 2];
        buffer[0] = self.read_register(start_register_address)?;
        buffer[1] = self.read_register(start_register_address + 1)?;
        self.combine_buffer_into_word(&buffer)
    }
}

impl<I2C> TFLunaSync for TFLuna<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    type Error = Error<I2C::Error>;

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
        self.i2c
            .write(self.address, &[constants::REBOOT_COMMAND_VALUE])
            .map_err(Error::I2c)?;
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

    /// Reads distance, signal strength, temperature and timestamp
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
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    fn test_read(register_address: u8, value: u8) -> Vec<I2cTransaction> {
        let expectations = Vec::from([
            I2cTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([register_address]),
            ),
            I2cTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([value])),
        ]);
        expectations
    }

    #[test]
    fn test_get_firmware_version() {
        let mut expectations = Vec::new();
        for i in 0..3 {
            expectations.extend_from_slice(&test_read(
                constants::FIRMWARE_VERSION_REGISTER_ADDRESS + i,
                i,
            ));
        }
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS, Delay {}).unwrap();
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
            expectations
                .extend_from_slice(&test_read(constants::SERIAL_NUMBER_REGISTER_ADDRESS + i, i));
        }
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS, Delay {}).unwrap();
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
            expectations
                .extend_from_slice(&test_read(constants::DISTANCE_REGISTER_ADDRESS + i, value));
        }
        // Signal Strength
        for (i, value) in (0..2).zip([0x64, 0]) {
            expectations.extend_from_slice(&test_read(
                constants::SIGNAL_STRENGTH_REGISTER_ADDRESS + i,
                value,
            ));
        }
        // Temperature
        for (i, value) in (0..2).zip([0xB2, 0x0C]) {
            expectations.extend_from_slice(&test_read(
                constants::TEMPERATURE_REGISTER_ADDRESS + i,
                value,
            ));
        }
        // Timestamp
        for (i, value) in (0..2).zip([0, 0]) {
            expectations
                .extend_from_slice(&test_read(constants::TIMESTAMP_REGISTER_ADDRESS + i, value));
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
