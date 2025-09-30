#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use embedded_hal_mock::eh1::delay::StdSleep as Delay;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cTraitMock, Transaction as I2cTraitTransaction};

    use embedded_tfluna::i2c::{Address, DEFAULT_SLAVE_ADDRESS, TFLuna};
    use embedded_tfluna::types::{FirmwareVersion, SensorReading, SerialNumber};

    /// Returns vector of i2c transaction expectations for an I2C read operation
    fn read_expectations(register_address: u8, value: &[u8]) -> Vec<I2cTraitTransaction> {
        let expectations = Vec::from([I2cTraitTransaction::write_read(
            DEFAULT_SLAVE_ADDRESS,
            Vec::from([register_address]),
            Vec::from(value),
        )]);
        expectations
    }

    /// Returns vector of i2c transaction expectations for an I2C write operation
    fn write_expectations(register_address: u8, value: u8) -> Vec<I2cTraitTransaction> {
        let expectations = Vec::from([I2cTraitTransaction::write(
            DEFAULT_SLAVE_ADDRESS,
            Vec::from([register_address, value]),
        )]);
        expectations
    }

    fn setup(i2c: &mut I2cTraitMock) -> TFLuna<&mut I2cTraitMock, Delay> {
        TFLuna::new(i2c, Address::default(), Delay {}).unwrap()
    }

    #[test]
    fn test_enable_disable() {
        let mut expectations = Vec::new();
        expectations.extend_from_slice(&write_expectations(0x25, 1));
        expectations.extend_from_slice(&write_expectations(0x25, 0));
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        assert!(device.enable().is_ok());
        assert!(device.disable().is_ok());
        i2c.done();
    }

    #[test]
    fn test_reboot() {
        let expectations = write_expectations(0x21, 2);
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        assert!(device.reboot().is_ok());
        i2c.done();
    }

    #[test]
    fn test_get_firmware_version() {
        let expectations = read_expectations(0x0A, &[0, 1, 2]);
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
        let expectations = read_expectations(0x10, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
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
        let expectations = read_expectations(0x00, &[10, 0, 0x64, 0, 0xB2, 0x0C, 0, 0, 0, 0]);
        let mut i2c = I2cTraitMock::new(&expectations);
        let mut device = setup(&mut i2c);
        let sensor_reading = device.get_measurement();
        assert!(sensor_reading.is_ok(), "{:?}", sensor_reading);
        let expected_sensor_reading = SensorReading {
            distance: 10,
            signal_strength: 100,
            temperature: 32.5,
            timestamp: 0,
            error: 0,
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
