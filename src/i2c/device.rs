use crate::i2c::constants;
use crate::i2c::types::{Address, Error, Register};

use crate::types::{
    FirmwareVersion, PowerMode, RangingMode, SensorReading, SerialNumber, Signature,
};

use super::{bisync, only_async, only_sync};

#[only_sync]
use embedded_hal::{
    delay::DelayNs,
    i2c::{Error as I2CError, ErrorKind, I2c as I2cTrait, SevenBitAddress},
};

#[only_async]
use embedded_hal_async::{
    delay::DelayNs,
    i2c::{Error as I2CError, ErrorKind, I2c as I2cTrait, SevenBitAddress},
};

/// TF-Luna controller
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    /// Associated method to create a new instance of the controller
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
    /// * `buffer`: Two-element array containing [low_byte, high_byte]
    ///
    /// # Returns
    /// * `u16`: Combined 16-bit value
    fn combine_buffer_into_word(&self, buffer: &[u8; 2]) -> u16 {
        buffer[0] as u16 + ((buffer[1] as u16) << 8)
    }

    #[bisync]
    async fn read<const N: usize>(
        &mut self,
        register: Register,
        buffer: &mut [u8; N],
    ) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write_read(self.address.into(), &[register as u8], buffer)
            .await
            .map_err(Error::I2c)?;
        Ok(())
    }

    #[bisync]
    async fn write<const N: usize>(&mut self, buffer: &[u8; N]) -> Result<(), Error<I2C::Error>> {
        self.i2c.write(self.address.into(), buffer).await?;
        Ok(())
    }

    /// Write byte to a single register
    #[bisync]
    async fn write_byte(
        &mut self,
        register: Register,
        content: u8,
    ) -> Result<(), Error<I2C::Error>> {
        self.write(&[register as u8, content]).await
    }

    /// Read the contents of a single register
    #[bisync]
    async fn read_byte(&mut self, register: Register) -> Result<u8, Error<I2C::Error>> {
        let mut buffer = [0; 1];
        self.read(register, &mut buffer).await?;
        Ok(buffer[0])
    }

    /// Read word (two bytes) from two consecutive registers
    ///
    /// # Arguments
    /// * `register`: Address of first register. Second's register's address will be `register + 1`
    ///
    /// # Returns
    /// * Ok(u16): Read and combined value from consecutive registers.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    #[bisync]
    async fn read_word(&mut self, register: Register) -> Result<u16, Error<I2C::Error>> {
        let mut buffer = [0; 2];
        self.read(register, &mut buffer).await?;
        Ok(self.combine_buffer_into_word(&buffer))
    }

    /// Write word (two bytes) into two consecutive registers.
    ///
    /// # Arguments
    /// * `register`: Address of first register. Second's register's address will be `register + 1`
    ///
    /// # Returns
    /// * Ok(()): if the write was successful.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// - The value is stored as a 16-bit value across two registers in little-endian order.
    /// - Low byte is written in register at start address.
    /// - High byte is written in register at start address + 1.
    #[bisync]
    async fn write_word(
        &mut self,
        register: Register,
        value: u16,
    ) -> Result<(), Error<I2C::Error>> {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;
        self.write(&[register as u8, low_byte, high_byte]).await
    }

    /// Restore all settings to factory defaults.
    ///
    /// # Notes
    /// This will reset all configurable parameters to their original values.
    #[bisync]
    pub async fn restore_factory_defaults(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            Register::RestoreFactoryDefaults,
            constants::RESTORE_FACTORY_DEFAULTS_COMMAND_VALUE,
        )
        .await
    }

    /// Save current settings to persistent storage.
    #[bisync]
    pub async fn save_settings(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::Save, constants::SAVE_COMMAND_VALUE)
            .await
    }

    /// Set enable bit.
    ///
    /// Calling this method will enable the device's measurements.
    #[bisync]
    pub async fn enable(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::Enable, constants::ENABLE_COMMAND_VALUE)
            .await
    }

    /// Unset enable bit
    ///
    /// Calling this method will disable the device's measurements.
    #[bisync]
    pub async fn disable(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::Enable, constants::DISABLE_COMMAND_VALUE)
            .await
    }

    /// Reboots device
    #[bisync]
    pub async fn reboot(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::ShutdownReboot, constants::REBOOT_COMMAND_VALUE)
            .await
    }

    /// Get the device firmware.
    ///
    /// # Returns
    /// * `Ok(FirmwareVersion)`: current firmware version.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    #[bisync]
    pub async fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Error<I2C::Error>> {
        let mut buffer = [0; 3];
        self.read(Register::FirmwareVersion, &mut buffer).await?;
        let version = FirmwareVersion {
            major: buffer[2],
            minor: buffer[1],
            revision: buffer[0],
        };
        Ok(version)
    }

    /// Get the device's serial number.
    ///
    /// # Returns
    /// * `Ok(SerialNumber)`: device serial number.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    #[bisync]
    pub async fn get_serial_number(&mut self) -> Result<SerialNumber, Error<I2C::Error>> {
        let mut buffer = [0; 14];
        self.read(Register::SerialNumber, &mut buffer).await?;
        Ok(SerialNumber(buffer))
    }

    /// Get the device's signature ('L' 'U' 'N' 'A').
    ///
    /// # Returns
    /// * `Ok(Signature)`: 4-byte ASCII signature.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    #[bisync]
    pub async fn get_signature(&mut self) -> Result<Signature, Error<I2C::Error>> {
        let mut buffer = [0; 4];
        self.read::<4>(Register::Signature, &mut buffer).await?;
        Ok(Signature(buffer))
    }

    /// Get the current I2C slave address of the device.
    ///
    /// # Returns
    /// * `Ok(u8)`: current slave address.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    #[bisync]
    pub async fn get_slave_address(&mut self) -> Result<u8, Error<I2C::Error>> {
        self.read_byte(Register::SlaveAddress).await
    }

    /// Set the I2C slave address of the device.
    ///
    /// # Arguments
    /// * `address`: New slave address.
    ///
    /// # Returns
    /// * `Ok(())`: if address was set successfully.
    /// * `Err(Error::InvalidParameter)`: if address is out of valid range.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Valid addresses are in the range [0x08, 0x77]
    /// * If you change the I2C slave address you will have to recreate an instance of [`TFLuna`]
    ///   with the new address.

    #[bisync]
    pub async fn set_slave_address(&mut self, address: u8) -> Result<(), Error<I2C::Error>> {
        if !(constants::SLAVE_ADDRESS_MINIMUM_VALUE..=constants::SLAVE_ADDRESS_MAXIMUM_VALUE)
            .contains(&address)
        {
            return Err(Error::InvalidParameter);
        }
        self.write_byte(Register::SlaveAddress, address).await
    }

    /// Get the current power mode of the device.
    ///
    /// # Returns
    /// * `Ok(PowerMode)`: current power mode.
    /// * `Err(Error::InvalidState)`: if registers contain invalid values,
    ///
    /// # Notes
    /// Reading registers will wake up the device from ultra-low power mode.
    /// Avoid frequent calls when ultra-low power mode is expected.

    #[bisync]
    pub async fn get_power_mode(&mut self) -> Result<PowerMode, Error<I2C::Error>> {
        let power_saving_mode_value = self.read_byte(Register::PowerSavingMode).await;

        match power_saving_mode_value {
            Ok(0x00) => Ok(PowerMode::Normal),
            Ok(0x01) => Ok(PowerMode::PowerSaving),
            Ok(val) => Err(Error::InvalidData(val)),
            Err(e) => {
                match e {
                    Error::<I2C::Error>::I2c(e) => {
                        // Check if the I2C error is a NoAcknowledge error
                        if let ErrorKind::NoAcknowledge(_) = e.kind() {
                            Ok(PowerMode::UltraLow)
                        } else {
                            // Return the original I2C error for other error kinds
                            Err(Error::I2c(e))
                        }
                    }
                    // All other errors
                    _ => Err(e),
                }
            }
        }
    }

    /// Set the power mode of the device.
    ///
    /// # Arguments
    /// * `mode`: desired power mode.
    ///
    /// # Notes
    /// * Power saving modes may reduce power consumption at the cost of performance.
    /// * Do not send setup commands while in ultra-low power mode.

    #[bisync]
    pub async fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error<I2C::Error>> {
        match mode {
            PowerMode::Normal => {
                self.disable_ultra_low_power_mode().await?;
                self.set_normal_power_mode().await?;
            }
            PowerMode::PowerSaving => {
                self.disable_ultra_low_power_mode().await?;
                self.set_power_saving_mode().await?;
            }
            PowerMode::UltraLow => {
                self.enable_ultra_low_power_mode().await?;
            }
        }
        self.delay.delay_ms(100).await;
        Ok(())
    }

    #[bisync]
    async fn set_normal_power_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            Register::PowerSavingMode,
            constants::NORMAL_POWER_MODE_COMMAND_VALUE,
        )
        .await
    }

    #[bisync]
    async fn set_power_saving_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(
            Register::PowerSavingMode,
            constants::POWER_SAVING_POWER_MODE_COMMAND_VALUE,
        )
        .await
    }

    // Set ultra-low power mode, save settings and reboot

    #[bisync]
    async fn enable_ultra_low_power_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write(&[
            Register::UltraLowPowerMode as u8,
            constants::ULTRA_LOWER_POWER_MODE_COMMAND_VALUE,
            constants::SAVE_COMMAND_VALUE,
            constants::REBOOT_COMMAND_VALUE,
        ])
        .await?;
        // Wait for a second for the device to be ready again
        self.delay.delay_ms(1000).await;
        Ok(())
    }

    #[bisync]
    async fn disable_ultra_low_power_mode(&mut self) -> Result<(), Error<I2C::Error>> {
        self.wake_from_ultra_low_power().await?;
        self.write::<4>(&[
            Register::UltraLowPowerMode as u8,
            constants::NORMAL_POWER_MODE_COMMAND_VALUE,
            constants::SAVE_COMMAND_VALUE,
            constants::REBOOT_COMMAND_VALUE,
        ])
        .await?;
        // Wait for a second for the device to be ready again
        self.delay.delay_ms(1000).await;
        Ok(())
    }

    /// Wakes up device from ultra-low power mode.
    ///
    /// # Notes
    /// * This is only useful in [`PowerMode::UltraLow`] power mode.
    /// * If that is the case, the method waits for 12ms before returning.
    /// * In other power modes, there is no delay.

    #[bisync]
    pub async fn wake_from_ultra_low_power(&mut self) -> Result<(), Error<I2C::Error>> {
        // Wake up by reading any register
        match self.read_byte(Register::Distance).await {
            Ok(_) => Ok(()),
            Err(e) => {
                match e {
                    Error::<I2C::Error>::I2c(e) => {
                        // Check if the I2C error is a NoAcknowledge error
                        if let ErrorKind::NoAcknowledge(_) = e.kind() {
                            // Wait at least 12ms after awakening as per manual
                            self.delay.delay_ms(12).await;
                            Ok(())
                        } else {
                            // Return the original I2C error for other error kinds
                            Err(Error::I2c(e))
                        }
                    }
                    _ => Err(Error::Other),
                }
            }
        }
    }

    /// Get the current ranging mode of the device.
    ///
    /// # Returns
    /// * `Ok(RangingMode)`: current ranging mode.
    /// * `Err(Error::InvalidData)`: if register contains invalid value.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn get_ranging_mode(&mut self) -> Result<RangingMode, Error<I2C::Error>> {
        let mode = self.read_byte(Register::RangingMode).await?;
        match mode {
            val if val == RangingMode::Continuous as u8 => Ok(RangingMode::Continuous),
            val if val == RangingMode::Trigger as u8 => Ok(RangingMode::Trigger),
            _ => Err(Error::<I2C::Error>::InvalidData(mode)),
        }
    }

    /// Set the ranging mode of the device.
    ///
    /// # Arguments
    /// * `mode`: desired ranging mode.
    ///
    /// # Returns
    /// * `Ok(())`: if ranging mode was set successfully.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// In [`RangingMode::Trigger`] mode, use [`TFLuna::trigger_measurement()`] to initiate measurements.

    #[bisync]
    pub async fn set_ranging_mode(&mut self, mode: RangingMode) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::RangingMode, mode as u8).await
    }

    /// Get the current measurement framerate in Hz.
    ///
    /// # Returns
    /// * `Ok(u16)` - current framerate in Hz
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error

    #[bisync]
    pub async fn get_framerate(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::Framerate).await
    }

    /// Set the measurement framerate in Hz.
    ///
    /// # Arguments
    /// * `framerate`: desired framerate in Hz (only valid values).
    ///
    /// # Returns
    /// * `Ok(())`: if framerate was set successfully.
    /// * `Err(Error::InvalidParameter)`: if framerate is invalid.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Only factors of 500Hz / n, where n in [2, 3, ...], are allowed.

    #[bisync]
    pub async fn set_framerate(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        match value {
            x if x == 0 || (x < 500 && (500 % x) == 0) => {
                self.write_word(Register::Framerate, value).await
            }
            _ => Err(Error::<I2C::Error>::InvalidParameter),
        }
    }

    /// Get the current signal strength threshold.
    ///
    /// # Returns
    /// * `Ok(u16)`: current signal strength threshold value.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// When Signal Strength < Signal Strength Threshold * 10,
    /// then the returned distance is the dummy distance instead of the actual distance

    #[bisync]
    pub async fn get_signal_strength_threshold(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::SignalStrengthThreshold).await
    }

    /// Set the signal strength threshold for valid measurements.
    ///
    /// # Arguments
    /// * `value`: minimum signal strength value for valid measurements.
    ///
    /// # Returns
    /// * `Ok(())`: if signal strength threshold was set successfully.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// When Signal Strength < Signal Strength Threshold * 10,
    /// then the returned distance is the dummy distance instead of the actual distance

    #[bisync]
    pub async fn set_signal_strength_threshold(
        &mut self,
        value: u16,
    ) -> Result<(), Error<I2C::Error>> {
        self.write_word(Register::SignalStrengthThreshold, value)
            .await
    }

    /// Get the current dummy distance value.
    ///
    /// # Returns
    /// * `Ok(u16)`: current dummy distance value.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn get_dummy_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::DummyDistance).await
    }

    /// Set the dummy distance value.
    ///
    /// # Arguments
    /// * `distance`: distance value to return when Signal Strength < Signal Strength Threshold * 10.
    ///
    /// # Returns
    /// * `Ok(())`: if dummy distances was set successfully.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn set_dummy_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(Register::DummyDistance, value).await
    }

    /// Get the current maximum distance setting.
    ///
    /// # Returns
    /// * `Ok(u16)`: current maximum distance.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn get_minimum_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::MinimumDistance).await
    }

    /// Set the minimum valid distance measurement
    ///
    /// # Arguments
    /// * `value`: minimum distance in millimeters.
    ///
    /// # Returns
    /// * `Ok(())`: if operation was successful.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Measurements below this distance may be filtered

    #[bisync]
    pub async fn set_minimum_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(Register::MinimumDistance, value).await
    }

    /// Get the current maximum distance setting.
    ///
    /// # Returns
    /// * `Ok(u16)`: current maximum distance.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn get_maximum_distance(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::MaximumDistance).await
    }

    /// Set the maximum valid distance measurement.
    ///
    /// # Arguments
    /// * `value`: maximum distance in millimeters.
    ///
    /// # Returns
    /// * `Ok(())`: if operation was successful.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Measurements above this distance may be filtered

    #[bisync]
    pub async fn set_maximum_distance(&mut self, value: u16) -> Result<(), Error<I2C::Error>> {
        self.write_word(Register::MaximumDistance, value).await
    }

    /// Get the error code from the device.
    ///
    /// # Returns
    /// * `Ok(u16)`: Error code value.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.

    #[bisync]
    pub async fn get_error(&mut self) -> Result<u16, Error<I2C::Error>> {
        self.read_word(Register::Error).await
    }

    /// Perform a complete measurement reading from the sensor.
    ///
    /// # Returns
    /// * `Ok(SensorReading)`: Structure containing distance, signal strength, temperature, timestamp and error.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Reads four 16-bit values from consecutive register pairs:
    ///   - Distance: Registers 0x00 (low byte) and 0x01 (high byte) in centimeters
    ///   - Signal Strength: Registers 0x02 (low byte) and 0x03 (high byte)
    ///   - Temperature: Registers 0x04 (low byte) and 0x05 (high byte) in 0.01°C units
    ///   - Timestamp: Registers 0x06 (low byte) and 0x07 (high byte) device ticks
    ///   - Error: Registers 0x08 (low byte) and 0x09 (high byte) error code
    ///
    /// * Temperature is automatically converted from hundredths of degrees Celsius to degrees Celsius.

    #[bisync]
    pub async fn get_measurement(&mut self) -> Result<SensorReading, Error<I2C::Error>> {
        let mut buffer = [0; 10];
        self.read::<10>(Register::Distance, &mut buffer).await?;
        let distance = self.combine_buffer_into_word(&[buffer[0], buffer[1]]);
        let signal_strength = self.combine_buffer_into_word(&[buffer[2], buffer[3]]);
        let temperature = self.combine_buffer_into_word(&[buffer[4], buffer[5]]);
        let temperature = temperature as f32 / 100.0;
        let timestamp = self.combine_buffer_into_word(&[buffer[6], buffer[7]]);
        let error = self.combine_buffer_into_word(&[buffer[8], buffer[9]]);
        Ok(SensorReading {
            distance,
            signal_strength,
            temperature,
            timestamp,
            error,
        })
    }

    /// Trigger a single measurement (only effective in [`RangingMode::Trigger`]).
    ///
    /// # Returns
    /// * `Ok(())`: if trigger was set successfully.
    /// * `Err(Error::I2c(I2CError))`: if there was an I2C error.
    ///
    /// # Notes
    /// * Only works when device is in [`RangingMode::Trigger`].
    /// * Initiates immediate measurement in trigger mode.

    #[bisync]
    pub async fn trigger_measurement(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_byte(Register::Trigger, constants::TRIGGER_COMMAND_VALUE)
            .await?;
        Ok(())
    }
}
