pub mod constants;
pub mod errors;

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, TenBitAddress};

use crate::i2c::errors::Error;
use crate::traits::{FirmwareVersion, SerialNumber, TFLunaSync};

#[derive(Debug)]
pub struct TFLuna<I2C: I2c<TenBitAddress>, D: DelayNs> {
    i2c: I2C,
    address: TenBitAddress,
    delay: D,
}

impl<I2C, D> TFLuna<I2C, D>
where
    I2C: I2c<TenBitAddress>,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: TenBitAddress, delay: D) -> Result<Self, Error<I2C::Error>> {
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
    I2C: I2c<TenBitAddress>,
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
