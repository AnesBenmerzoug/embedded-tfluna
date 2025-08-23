pub mod constants;
pub mod errors;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::i2c::errors::Error;
use crate::traits::{FirmwareVersion, SerialNumber, TFLunaSync};

#[derive(Debug)]
pub struct TFLuna<I2C: I2c<SevenBitAddress>, D: DelayNs> {
    i2c: I2C,
    address: SevenBitAddress,
    delay: D,
}

impl<I2C, D> TFLuna<I2C, D>
where
    I2C: I2c<SevenBitAddress>,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: SevenBitAddress, delay: D) -> Result<Self, Error<I2C::Error>> {
        let sensor = Self {
            i2c,
            address,
            delay,
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
}

impl<I2C, D> TFLunaSync for TFLuna<I2C, D>
where
    I2C: I2c<SevenBitAddress>,
    D: DelayNs,
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
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::println;
    use std::vec::Vec;

    use super::*;
    use embedded_hal_mock::eh1::delay::StdSleep as Delay;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[test]
    fn test_get_firmware_version() {
        let expectations = [
            I2cTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([constants::FIRMWARE_VERSION_REGISTER_ADDRESS]),
            ),
            I2cTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([0])),
            I2cTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([constants::FIRMWARE_VERSION_REGISTER_ADDRESS + 1]),
            ),
            I2cTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([1])),
            I2cTransaction::write(
                constants::DEFAULT_SLAVE_ADDRESS,
                Vec::from([constants::FIRMWARE_VERSION_REGISTER_ADDRESS + 2]),
            ),
            I2cTransaction::read(constants::DEFAULT_SLAVE_ADDRESS, Vec::from([1])),
        ];
        let mut i2c = I2cMock::new(&expectations);
        let mut device = TFLuna::new(&mut i2c, constants::DEFAULT_SLAVE_ADDRESS, Delay {}).unwrap();
        let firmware_version = device.get_firmware_version().map_err(|err| {
            println!("error: {:?}", err);
            err
        });
        assert!(firmware_version.is_ok(), "{:?}", firmware_version);
        let expected_firmware_version = FirmwareVersion {
            major: 1,
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
}
