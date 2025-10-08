#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_tfluna::i2c::{Address, asynchronous::TFLuna};
use esp_hal::clock::CpuClock;
use esp_hal::timer::{OneShotTimer, systimer::SystemTimer, timg::TimerGroup};
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

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) -> ! {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // I2C SDA (Data) Pin
    let sda_pin = peripherals.GPIO8;
    // I2C SCL (Clock) Pin
    let scl_pin = peripherals.GPIO9;
    let i2c_config = I2cConfig::default().with_frequency(Rate::from_khz(100));
    let i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(sda_pin)
        .with_scl(scl_pin)
        .into_async();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let timer = OneShotTimer::new(timg0.timer0).into_async();
    let mut tfluna: TFLuna<_, _> = TFLuna::new(i2c, Address::default(), timer).unwrap();

    // Restore factory defaults and then reboot device
    tfluna.restore_factory_defaults().await.unwrap();
    tfluna.reboot().await.unwrap();
    Timer::after(Duration::from_millis(500)).await;

    // Enable measurements
    tfluna.enable().await.unwrap();

    loop {
        let measurement = tfluna.get_measurement().await.unwrap();
        info!("Distance = {:?}", measurement.distance);
        Timer::after(Duration::from_millis(500)).await;
    }
}
