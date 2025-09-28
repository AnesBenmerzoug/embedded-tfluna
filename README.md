# Embedded TF-Luna

[crates-badge]: https://img.shields.io/crates/v/embedded-tfluna.svg
[crates-url]: https://crates.io/crates/embedded-tfluna
[docs-badge]: https://docs.rs/embedded-tfluna/badge.svg
[docs-url]: https://docs.rs/embedded-tfluna
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?labelColor=1C2C2E&style=flat-square
[ci-badge]: https://github.com/AnesBenmerzoug/embedded-tfluna/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/AnesBenmerzoug/embedded-tfluna/actions?query=workflow%3ACI+branch%3Amain

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
![MIT/Apache-2.0 licensed][license-badge]
[![Build Status][ci-badge]][ci-url]

> Platform agnostic Rust driver for the [TF-Luna] LiDAR distance sensor, based on the [embedded-hal] traits.

[TF-Luna]: https://en.benewake.com/TFLuna/index.html
[embedded-hal]: https://github.com/rust-embedded/embedded-hal

This library provides a `no_std` interface for interacting with the [TF-Luna] LiDAR distance sensor.

The TF-Luna supports both I2C and UART communication protocols.
However, this library only supports I2C for now.

## Getting Started

### Usage

Check out the [examples](examples/) directory for examples of using the TF-Luna
with different micro-controllers and boards.

#### Example for ESP32 C3

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embedded_tfluna::i2c::{Address, TFLuna};
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
    let mut tfluna: TFLuna<_, _> = TFLuna::new(i2c, Address::default(), Delay::new()).unwrap();

    // Enable measurements
    tfluna.enable().unwrap();

    loop {
        let measurement = tfluna.measure().unwrap();
        info!("Distance = {:?}", measurement.distance);
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }
}
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
