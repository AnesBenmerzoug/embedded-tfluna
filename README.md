# Embedded TF-Luna

This Rust library provides a `no_std` interface for interacting with the
[TF-Luna](https://en.benewake.com/TFLuna/index.html)
LiDAR distance sensor.

The TF-Luna supports both I2C and UART communication protocols.

For now, only I2C is implemented.

## Getting Started

### Usage

Check out the [examples](examples/) directory for examples of using the TF-Luna
with different micro-controllers and boards.

#### Example for ESP32 C3

```rust
#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::{
    i2c::master::{Config, I2c},
    time::Rate,
};
use embedded_tfluna::i2c::{TFLuna, DEFAULT_SLAVE_ADDRESS};

fn main() -> ! {
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
    let mut tfluna: TFLuna<_, _> =
        TFLuna::new(i2c, DEFAULT_SLAVE_ADDRESS, Delay::new()).unwrap();

    loop {
        let measurement = tfluna.measure().unwrap();
        
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
