#![no_std]
#![no_main]

use esp_hal as _;
// use panic_rtt_target as _;

#[cfg(test)]
#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]
mod tests {
    use esp_hal::clock::CpuClock;
    use esp_hal::delay::Delay;
    use esp_hal::gpio::{Input, InputConfig, Pull};
    use esp_hal::{
        i2c::master::{Config, I2c},
        time::Rate,
    };
    use log::debug;

    use embedded_tfluna::{
        i2c::{Address, DEFAULT_SLAVE_ADDRESS, TFLuna},
        types::{FirmwareVersion, PowerMode, RangingMode, SerialNumber, Signature},
    };

    struct Context {
        tfluna: TFLuna<I2c<'static, esp_hal::Blocking>, Delay>,
        delay: Delay,
        data_ready_pin: Input<'static>,
    }

    // init function which is called before every test
    #[init]
    fn init() -> Context {
        debug!("Initialization");
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
        let mut tfluna: TFLuna<_, _> = TFLuna::new(i2c, Address::default(), Delay::new()).unwrap();
        let delay = Delay::new();
        let data_ready_pin = Input::new(
            peripherals.GPIO10,
            InputConfig::default().with_pull(Pull::None),
        );

        // Set power mode to Normal, mostly in case we are in ultra-low power mode
        // We try multiple times in case waking up from ultra-low power mode fails
        for _ in 0..3 {
            let _ = tfluna.set_power_mode(PowerMode::Normal);
        }
        delay.delay_millis(1000);
        tfluna.set_power_mode(PowerMode::Normal).unwrap();

        // Make sure we are in normal power mode
        assert_eq!(tfluna.get_power_mode().unwrap(), PowerMode::Normal);

        // Restore factory defaults and then reboot device
        // After sending the reboot command we wait for a bit
        // until the device stabilizes
        tfluna.restore_factory_defaults().unwrap();
        tfluna.reboot().unwrap();
        delay.delay_millis(1000);

        Context {
            tfluna,
            delay,
            data_ready_pin,
        }
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
        assert_eq!(
            firmware_version,
            FirmwareVersion {
                major: 3,
                minor: 5,
                revision: 1
            }
        );
    }

    #[test]
    fn test_get_serial_number(context: Context) {
        let mut tfluna = context.tfluna;
        let serial_number = tfluna.get_serial_number().unwrap();
        assert_eq!(
            serial_number,
            SerialNumber([84, 51, 51, 48, 48, 50, 52, 53, 48, 49, 48, 48, 56, 50])
        );
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
        let slave_address = tfluna.get_slave_address().unwrap();
        assert_eq!(slave_address, DEFAULT_SLAVE_ADDRESS);
    }

    #[test]
    fn test_power_mode(context: Context) {
        let mut tfluna = context.tfluna;

        debug!("Reading default power mode");
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::Normal);

        debug!("Transitioning power mode from Normal to Power Saving");
        tfluna.set_power_mode(PowerMode::PowerSaving).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::PowerSaving);

        debug!("Transitioning power mode from Power Saving to Normal");
        tfluna.set_power_mode(PowerMode::Normal).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::Normal);

        debug!("Transitioning power mode from Normal to Ultra-low");
        tfluna.set_power_mode(PowerMode::UltraLow).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::UltraLow);

        debug!("Transitioning power mode from Ultra-low to Normal");
        let _ = tfluna.set_power_mode(PowerMode::Normal);
        let _ = tfluna.set_power_mode(PowerMode::Normal);
        tfluna.set_power_mode(PowerMode::Normal).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::Normal);

        debug!("Transitioning power mode from Normal to Power Saving");
        tfluna.set_power_mode(PowerMode::PowerSaving).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::PowerSaving);

        debug!("Transitioning power mode from Power Saving to Ultra-low");
        tfluna.set_power_mode(PowerMode::UltraLow).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::UltraLow);

        debug!("Transitioning power mode from Ultra-low to Power Saving");
        let _ = tfluna.set_power_mode(PowerMode::PowerSaving);
        let _ = tfluna.set_power_mode(PowerMode::PowerSaving);
        tfluna.set_power_mode(PowerMode::PowerSaving).unwrap();
        context.delay.delay_millis(100);
        let power_mode = tfluna.get_power_mode().unwrap();
        assert_eq!(power_mode, PowerMode::PowerSaving);
    }

    #[test]
    fn test_ranging_mode(context: Context) {
        let mut tfluna = context.tfluna;
        // Get ranging mode and expect it to be set to Continuous by default
        let ranging_mode = tfluna.get_ranging_mode().unwrap();
        assert_eq!(ranging_mode, RangingMode::Continuous);
        // Set ranging mode to trigger and expect it to be set
        tfluna.set_ranging_mode(RangingMode::Trigger).unwrap();
        let ranging_mode = tfluna.get_ranging_mode().unwrap();
        assert_eq!(ranging_mode, RangingMode::Trigger);
    }

    #[test]
    fn test_framerate(context: Context) {
        let mut tfluna = context.tfluna;
        // Get framerate and expect it to be set to default value
        let framerate = tfluna.get_framerate().unwrap();
        debug!("Framerate before change = {}", framerate);
        assert_eq!(framerate, 100);
        // Set framerate to anohter value and expect it to be set
        let new_framerate = 250;
        tfluna.set_framerate(new_framerate).unwrap();
        context.delay.delay_millis(100);
        let framerate = tfluna.get_framerate().unwrap();
        debug!("Framerate after change = {}", framerate);
        assert_eq!(framerate, new_framerate);
        context.delay.delay_millis(10);
    }

    #[test]
    fn test_signal_strength_threshold(context: Context) {
        let mut tfluna = context.tfluna;
        // Get signal strength threshold and expect it to be set to default value
        let signal_strength_threshold = tfluna.get_signal_strength_threshold().unwrap();
        assert_eq!(signal_strength_threshold, 100);
        // Set signal strength threshold to another value and expect it to be set
        let new_signal_strength_threshold = 600;
        tfluna
            .set_signal_strength_threshold(new_signal_strength_threshold)
            .unwrap();
        let signal_strength_threshold = tfluna.get_signal_strength_threshold().unwrap();
        assert_eq!(signal_strength_threshold, new_signal_strength_threshold);
    }

    #[test]
    fn test_dummy_distance(context: Context) {
        let mut tfluna = context.tfluna;
        // Get dummy distance and expect it to be set to default value
        let dummy_distance = tfluna.get_dummy_distance().unwrap();
        assert_eq!(dummy_distance, 0);
        // Set dummy distance to another value and expect it to be set
        let new_dummy_distance = 66;
        tfluna.set_dummy_distance(new_dummy_distance).unwrap();
        let dummy_distance = tfluna.get_dummy_distance().unwrap();
        assert_eq!(dummy_distance, new_dummy_distance)
    }

    #[test]
    fn test_minimum_distance(context: Context) {
        let mut tfluna = context.tfluna;
        // Get minimum distance and expect it to be set to default value
        let minimum_distance = tfluna.get_minimum_distance().unwrap();
        assert_eq!(minimum_distance, 0);
        // Set minimum distance to another value and expect it to be set
        let new_minimum_distance = 66;
        tfluna.set_minimum_distance(new_minimum_distance).unwrap();
        let minimum_distance = tfluna.get_minimum_distance().unwrap();
        assert_eq!(minimum_distance, new_minimum_distance)
    }

    #[test]
    fn test_maximum_distance(context: Context) {
        let mut tfluna = context.tfluna;
        // Get maximum distance and expect it to be set to default value
        let maximum_distance = tfluna.get_maximum_distance().unwrap();
        assert_eq!(maximum_distance, 9000);
        // Set maximum distance to another value and expect it to be set
        let new_maximum_distance = 2000;
        tfluna.set_maximum_distance(new_maximum_distance).unwrap();
        let maximum_distance = tfluna.get_maximum_distance().unwrap();
        assert_eq!(maximum_distance, new_maximum_distance)
    }

    #[test]
    fn test_error(context: Context) {
        let mut tfluna = context.tfluna;
        // Get error and expect it to be set to default value
        let error = tfluna.get_error().unwrap();
        assert_eq!(error, 0);
    }

    #[test]
    fn test_measure(context: Context) {
        let mut tfluna = context.tfluna;
        // We take an initial measurement and make sure all values have appropriate values
        let measurement = tfluna.get_measurement().unwrap();
        debug!("measurement = {:?}", measurement);
        context.delay.delay_millis(1);

        assert!(measurement.distance > 0);
        assert!(measurement.signal_strength > 0);
        assert!(measurement.temperature > 0.0);
        assert!(measurement.timestamp > 0);
        assert_eq!(measurement.error, 0);
        // We wait for a bit and take a second measurement and expect both to be different
        context.delay.delay_millis(100);
        let second_measurement = tfluna.get_measurement().unwrap();
        assert_ne!(measurement, second_measurement)
    }

    #[test]
    fn test_continuous_ranging_mode(context: Context) {
        let mut tfluna = context.tfluna;

        debug!("Set ranging mode to continuous");
        tfluna.set_ranging_mode(RangingMode::Continuous).unwrap();
        context.delay.delay_millis(500);
        assert_eq!(tfluna.get_ranging_mode().unwrap(), RangingMode::Continuous);

        debug!("Waiting for data ready pin to go high");
        for i in 0..10 {
            if context.data_ready_pin.is_high() {
                debug!("data ready pin is high after {}ms", i);
                break;
            }
            context.delay.delay_millis(1);
        }
        assert!(context.data_ready_pin.is_high());

        debug!("Reading measurement");
        tfluna.get_measurement().unwrap();

        debug!("Waiting for data ready pin to go low");
        for i in 0..10 {
            if context.data_ready_pin.is_low() {
                debug!("data ready pin is low after {}ms", i);
                break;
            }
            context.delay.delay_millis(1);
        }
        assert!(context.data_ready_pin.is_low());
    }

    #[test]
    fn test_trigger_ranging_mode(context: Context) {
        let mut tfluna = context.tfluna;

        debug!("Taking initial measurement");
        let initial_measurement = tfluna.get_measurement().unwrap();
        context.delay.delay_millis(100);

        debug!("Setting ranging mode to trigger");
        tfluna.set_ranging_mode(RangingMode::Trigger).unwrap();
        context.delay.delay_millis(100);
        assert_eq!(tfluna.get_ranging_mode().unwrap(), RangingMode::Trigger);

        // We trigger the measurement, wait a bit and then read the measured values
        debug!("Triggering measurement");
        tfluna.trigger_measurement().unwrap();
        context.delay.delay_millis(100);

        debug!("Reading measurement");
        let first_measurement_after_trigger = tfluna.get_measurement().unwrap();
        assert_ne!(initial_measurement, first_measurement_after_trigger);
        // We wait for some time and read again but without triggering the measurement
        context.delay.delay_millis(100);
        let second_measurement_after_trigger = tfluna.get_measurement().unwrap();
        assert_eq!(
            first_measurement_after_trigger,
            second_measurement_after_trigger
        );
    }

    // This test is ignored for now because I couldn't make it work
    // The ultra-low power mode seems to be very finicky.
    #[test]
    #[ignore]
    fn test_ultra_low_power_mode_read_measurement(context: Context) {
        let mut tfluna = context.tfluna;

        context.delay.delay_millis(1000);

        debug!("Setting ultra-low power mode");
        tfluna.set_power_mode(PowerMode::UltraLow).unwrap();

        context.delay.delay_millis(1000);

        debug!("Waking up from ultra lower power sleep");
        tfluna.wake_from_ultra_low_power().unwrap();

        debug!("Reading distance");
        let measurement = tfluna.get_measurement().unwrap();
        debug!("Measurement = {:?}", measurement);

        debug!("Setting power mode back to normal");
        tfluna.set_power_mode(PowerMode::Normal).unwrap();
        context.delay.delay_millis(1);
    }
}
