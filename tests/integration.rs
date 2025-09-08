#![no_std]
#![no_main]

use esp_hal as _;
use rtt_target::rtt_init_log;

/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.
/// Note: you can also inline this method in the attribute. e.g. `#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]`
fn setup_log() {
    rtt_init_log!();
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::setup_log())]
mod tests {
    use super::*;
    use esp_hal::delay::Delay;
    use esp_hal::clock::CpuClock;
    use esp_hal::{i2c::master::{I2c, Config}, time::Rate};
    use embedded_hal::i2c::AddressMode;
    use rtt_target::rprintln;

    use embedded_tfluna::{TFLuna, TFLunaSync, DEFAULT_SLAVE_ADDRESS, FirmwareVersion, Signature, SerialNumber, PowerMode, RangingMode};

    struct Context {
        #[allow(dead_code)]
        tfluna: TFLuna<I2c<'static, esp_hal::Blocking>, Delay>,
    }

    // init function which is called before every test
    #[init]
    fn init() -> Context {
        rprintln!("Initialization");
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        // I2C SDA (Data) Pin
        let sda_pin = peripherals.GPIO8;
        // I2C SCL (Clock) Pin
        let scl_pin = peripherals.GPIO9;
        let i2c_config = Config::default().with_frequency(Rate::from_khz(100));
        let i2c = I2c::new(peripherals.I2C0, i2c_config)
            .unwrap()
            .with_sda(sda_pin)
            .with_scl(scl_pin);
        let mut tfluna: TFLuna<_, _> = TFLuna::new(i2c, DEFAULT_SLAVE_ADDRESS, Delay::new()).unwrap();
        // Restore factory defaults and then reboot devicec
        tfluna.restore_factory_defaults().unwrap();
        tfluna.reboot().unwrap();
        Delay::new().delay_millis(500);
        Context { tfluna }
    }

    #[test]
    fn test_enable_disable(context: Context) {
        let mut tfluna = context.tfluna;
        tfluna.disable().unwrap();
        tfluna.enable().unwrap();
    }

    #[test]
    fn test_reboot(context: Context) {
        let mut tfluna = context.tfluna;
        tfluna.reboot().unwrap();
    }

    #[test]
    fn test_get_firmware_version(context: Context) {
        let mut tfluna = context.tfluna;
        let firmware_version = tfluna.get_firmware_version().unwrap();
        assert_eq!(firmware_version, FirmwareVersion {
            major: 3,
            minor: 5,
            revision: 1
        });
    }

    #[test]
    fn test_get_serial_number(context: Context) {
        let mut tfluna = context.tfluna;
        let serial_number = tfluna.get_serial_number().unwrap();
        assert_eq!(serial_number, SerialNumber([84, 51, 51, 48, 48, 50, 52, 53, 48, 49, 48, 48, 56, 50]));
    }

    #[test]
    fn test_get_signature(context: Context) {
        let mut tfluna = context.tfluna;
        let signature = tfluna.get_signature().unwrap();
        // Expected: 'L': 76, 'U': 85, 'N': 78, 'A': 65
        assert_eq!(signature, Signature([76, 85, 78, 65]));
    }

    #[test]
    fn test_get_i2c_slave_address(context: Context) {
        let mut tfluna = context.tfluna;
        let slave_address = tfluna.get_i2c_slave_address().unwrap();
        assert_eq!(slave_address, DEFAULT_SLAVE_ADDRESS);
    }

    #[test]
    fn test_power_mode(context: Context) {
        let mut tfluna = context.tfluna;
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::Normal);
    }

    #[test]
    fn test_ranging_mode(context: Context) {
        let mut tfluna = context.tfluna;
        // Get ranging mode and expect it to be set to be Continuous by default
        let ranging_mode = tfluna.get_ranging_mode().unwrap();
        assert_eq!(ranging_mode, RangingMode::Continuous);
        // Set ranging mode to trigger and expect it to be set
        tfluna.set_ranging_mode(RangingMode::Trigger).unwrap();
        let ranging_mode = tfluna.get_ranging_mode().unwrap();
        assert_eq!(ranging_mode, RangingMode::Trigger);
    }

    #[test]
    fn test_measure(context: Context) {
        let mut tfluna = context.tfluna;
        let measurement = tfluna.measure().unwrap();
        assert!(measurement.distance > 0);
        assert!(measurement.signal_strength > 0);
        assert!(measurement.temperature > 0.0);
        assert!(measurement.timestamp > 0);
    }
}