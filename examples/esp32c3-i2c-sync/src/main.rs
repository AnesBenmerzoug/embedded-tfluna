#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embedded_tfluna::{TFLunaSync, TFLuna, DEFAULT_SLAVE_ADDRESS};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::{
    i2c::master::{Config as I2cConfig, I2c},
    time::Rate,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // I2C SDA (Data) Pin
    let sda_pin = peripherals.GPIO8;
    // I2C SCL (Clock) Pin
    let scl_pin = peripherals.GPIO9;
    let i2c_config = I2cConfig::default().with_frequency(Rate::from_khz(100));
    let i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(sda_pin)
        .with_scl(scl_pin);
    let mut tfluna: TFLuna<_, _> = TFLuna::new(i2c, DEFAULT_SLAVE_ADDRESS, Delay::new()).unwrap();

    // Restore factory defaults and then reboot device
    tfluna.restore_factory_defaults().unwrap();
    tfluna.reboot().unwrap();
    Delay::new().delay_millis(500);

    // Enable measurements
    tfluna.enable().unwrap();

    loop {
        let measurement = tfluna.measure().unwrap();
        info!("Distance = {:?}", measurement.distance);
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
