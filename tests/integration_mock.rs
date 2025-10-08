#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    use embedded_hal_mock::eh1::delay::StdSleep as Delay;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cTraitMock, Transaction as I2cTraitTransaction};

    #[cfg(feature = "async")]
    use embedded_tfluna::i2c::asynchronous::TFLuna as TFLunaAsync;
    use embedded_tfluna::i2c::blocking::TFLuna as TFLunaBlocking;
    use embedded_tfluna::i2c::{Address, DEFAULT_SLAVE_ADDRESS, Error};
    use embedded_tfluna::{FirmwareVersion, SensorReading, SerialNumber};

    use rstest::*;

    enum Transaction<'a> {
        Write(u8, &'a [u8]),
        Read(u8, &'a [u8]),
    }

    impl<'a> From<Transaction<'a>> for I2cTraitTransaction {
        fn from(transaction: Transaction) -> Self {
            match transaction {
                Transaction::Read(register_address, value) => I2cTraitTransaction::write_read(
                    DEFAULT_SLAVE_ADDRESS,
                    Vec::from([register_address]),
                    Vec::from(value),
                ),
                Transaction::Write(register_address, value) => {
                    let mut write_data = Vec::from([register_address]);
                    let mut values = Vec::from(value);
                    write_data.append(&mut values);
                    I2cTraitTransaction::write(DEFAULT_SLAVE_ADDRESS, write_data)
                }
            }
        }
    }

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

    fn setup(i2c: &mut I2cTraitMock) -> TFLunaBlocking<&mut I2cTraitMock, Delay> {
        TFLunaBlocking::new(i2c, Address::default(), Delay {}).unwrap()
    }

    fn i2c_blocking(transactions: Vec<Transaction>) -> I2cTraitMock {
        let expectations = transactions
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<I2cTraitTransaction>>();
        I2cTraitMock::new(&expectations)
    }

    fn device_blocking(i2c: &mut I2cTraitMock) -> TFLunaBlocking<&mut I2cTraitMock, Delay> {
        TFLunaBlocking::new(i2c, Address::default(), Delay {}).unwrap()
    }

    #[cfg(feature = "async")]
    fn i2c_async(transactions: Vec<Transaction>) -> I2cTraitMock {
        let expectations = transactions
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<I2cTraitTransaction>>();
        I2cTraitMock::new(&expectations)
    }

    #[cfg(feature = "async")]
    fn device_async(i2c: &mut I2cTraitMock) -> TFLunaAsync<&mut I2cTraitMock, Delay> {
        TFLunaAsync::new(i2c, Address::default(), Delay {}).unwrap()
    }

    #[rstest]
    #[case::enable_then_disable(&mut i2c_blocking(Vec::from([
        Transaction::Write(0x25, &[1]),
        Transaction::Write(0x25, &[0]),
    ])))]
    fn test_enable_disable_blocking(#[case] i2c: &mut I2cTraitMock) {
        let mut device = device_blocking(i2c);
        assert!(device.enable().is_ok());
        assert!(device.disable().is_ok());
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::enable_then_disable(&mut i2c_blocking(Vec::from([
        Transaction::Write(0x25, &[1]),
        Transaction::Write(0x25, &[0]),
    ])))]
    async fn test_enable_disable_async(#[case] i2c: &mut I2cTraitMock) {
        let mut device = device_async(i2c);
        assert!(device.enable().await.is_ok());
        assert!(device.disable().await.is_ok());
        i2c.done();
    }

    #[rstest]
    #[case::reboot(&mut i2c_blocking(Vec::from([
        Transaction::Write(0x21, &[2]),
    ])))]
    fn test_reboot_blocking(#[case] i2c: &mut I2cTraitMock) {
        let mut device = device_blocking(i2c);
        assert!(device.reboot().is_ok());
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::reboot(&mut i2c_async(Vec::from([
        Transaction::Write(0x21, &[2]),
    ])))]
    async fn test_reboot_async(#[case] i2c: &mut I2cTraitMock) {
        let mut device = device_async(i2c);
        assert!(device.reboot().await.is_ok());
        i2c.done();
    }

    #[rstest]
    #[case::firmware_2_1_0(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x0A, &[0, 1, 2]),
    ])), FirmwareVersion { major: 2, minor: 1, revision: 0 })]
    #[case::firmware_1_2_3(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x0A, &[3, 2, 1]),
    ])), FirmwareVersion { major: 1, minor: 2, revision: 3 })]
    fn test_get_firmware_version_blocking(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_firmware_version: FirmwareVersion,
    ) {
        let mut device = device_blocking(i2c);
        let firmware_version = device.get_firmware_version();
        assert!(firmware_version.is_ok(), "{:?}", firmware_version);
        assert_eq!(
            firmware_version.unwrap(),
            expected_firmware_version,
            "{:?} is different from {:?}",
            firmware_version,
            expected_firmware_version
        );
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::firmware_2_1_0(&mut i2c_async(Vec::from([
        Transaction::Read(0x0A, &[0, 1, 2]),
    ])), FirmwareVersion { major: 2, minor: 1, revision: 0 })]
    #[tokio::test]
    #[case::firmware_1_2_3(&mut i2c_async(Vec::from([
        Transaction::Read(0x0A, &[3, 2, 1]),
    ])), FirmwareVersion { major: 1, minor: 2, revision: 3 })]
    async fn test_get_firmware_version_async(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_firmware_version: FirmwareVersion,
    ) {
        let mut device = device_async(i2c);
        let firmware_version = device.get_firmware_version().await;
        assert!(firmware_version.is_ok(), "{:?}", firmware_version);
        assert_eq!(
            firmware_version.unwrap(),
            expected_firmware_version,
            "{:?} is different from {:?}",
            firmware_version,
            expected_firmware_version
        );
        i2c.done();
    }

    #[rstest]
    #[case::all_zeros(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x10, &[0; 14]),
    ])), SerialNumber([0; 14]))]
    #[case::from_0_to_13(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x10, &core::array::from_fn::<u8, 14, _>(|i| i as u8 + 1)),
    ])), SerialNumber(core::array::from_fn::<u8, 14, _>(|i| i as u8 + 1)))]
    fn test_get_serial_number_blocking(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_serial_number: SerialNumber,
    ) {
        let mut device = device_blocking(i2c);
        let serial_number = device.get_serial_number();
        assert!(serial_number.is_ok(), "{:?}", serial_number);
        assert_eq!(
            serial_number.unwrap(),
            expected_serial_number,
            "{:?} is different from {:?}",
            serial_number,
            expected_serial_number
        );
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::all_zeros(&mut i2c_async(Vec::from([
        Transaction::Read(0x10, &[0; 14]),
    ])), SerialNumber([0; 14]))]
    #[tokio::test]
    #[case::from_0_to_13(&mut i2c_async(Vec::from([
        Transaction::Read(0x10, &core::array::from_fn::<u8, 14, _>(|i| i as u8 + 1)),
    ])), SerialNumber(core::array::from_fn::<u8, 14, _>(|i| i as u8 + 1)))]
    async fn test_get_serial_number_async(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_serial_number: SerialNumber,
    ) {
        let mut device = device_async(i2c);
        let serial_number = device.get_serial_number().await;
        assert!(serial_number.is_ok(), "{:?}", serial_number);
        assert_eq!(
            serial_number.unwrap(),
            expected_serial_number,
            "{:?} is different from {:?}",
            serial_number,
            expected_serial_number
        );
        i2c.done();
    }

    #[rstest]
    #[case::some_measurement(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x00, &[10, 0, 0x64, 0, 0xB2, 0x0C, 0, 0, 0, 0]),
    ])), SensorReading {
            distance: 10,
            signal_strength: 100,
            temperature: 32.5,
            timestamp: 0,
            error: 0,
    })]
    fn test_measure_blocking(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_measurement: SensorReading,
    ) {
        let mut device = device_blocking(i2c);
        let measurement = device.get_measurement();
        assert!(measurement.is_ok(), "{:?}", measurement);
        assert_eq!(
            measurement.unwrap(),
            expected_measurement,
            "{:?} is different from {:?}",
            measurement,
            expected_measurement
        );
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::some_measurement(&mut i2c_blocking(Vec::from([
        Transaction::Read(0x00, &[10, 0, 0x64, 0, 0xB2, 0x0C, 0, 0, 0, 0]),
    ])), SensorReading {
            distance: 10,
            signal_strength: 100,
            temperature: 32.5,
            timestamp: 0,
            error: 0,
    })]
    async fn test_measure_async(
        #[case] i2c: &mut I2cTraitMock,
        #[case] expected_measurement: SensorReading,
    ) {
        let mut device = device_async(i2c);
        let measurement = device.get_measurement().await;
        assert!(measurement.is_ok(), "{:?}", measurement);
        assert_eq!(
            measurement.unwrap(),
            expected_measurement,
            "{:?} is different from {:?}",
            measurement,
            expected_measurement
        );
        i2c.done();
    }

    #[rstest]
    #[case::framerate_240(&mut i2c_blocking(Vec::new()), 240)]
    #[case::framerate_500(&mut i2c_blocking(Vec::new()), 500)]
    fn test_invalid_framerate_blocking(#[case] i2c: &mut I2cTraitMock, #[case] framerate: u16) {
        let mut device = device_blocking(i2c);
        assert!(device.set_framerate(framerate).is_err());
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::framerate_240(&mut i2c_async(Vec::new()), 240)]
    #[tokio::test]
    #[case::framerate_500(&mut i2c_async(Vec::new()), 500)]
    async fn test_invalid_framerate_async(#[case] i2c: &mut I2cTraitMock, #[case] framerate: u16) {
        let mut device = device_async(i2c);
        assert!(device.set_framerate(framerate).await.is_err());
        i2c.done();
    }

    #[rstest]
    #[case::address_1(&mut i2c_blocking(Vec::new()), 1)]
    #[case::address_200(&mut i2c_blocking(Vec::new()), 200)]
    fn test_invalid_slave_address_blocking(#[case] i2c: &mut I2cTraitMock, #[case] address: u8) {
        let mut device = device_blocking(i2c);
        assert!(device.set_slave_address(address).is_err());
        i2c.done();
    }

    #[cfg(feature = "async")]
    #[rstest]
    #[tokio::test]
    #[case::address_1(&mut i2c_async(Vec::new()), 1)]
    #[tokio::test]
    #[case::address_200(&mut i2c_async(Vec::new()), 200)]
    async fn test_invalid_slave_address_async(#[case] i2c: &mut I2cTraitMock, #[case] address: u8) {
        let mut device = device_async(i2c);
        assert!(device.set_slave_address(address).await.is_err());
        i2c.done();
    }
}
