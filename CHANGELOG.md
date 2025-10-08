# Changelog

## [0.2.0] - 2025-10-08

[0.2.0]: https://github.com/AnesBenmerzoug/embedded-tfluna/releases/tag/v0.2.0

### Added

- Asynchronous I2C interface.
- Asynchronous I2C ESP32 C3 example.

### Changed

- **Breaking** moved previous blocking implementation into `blocking` module inside existing`i2c` module.
  The re-export of the blocking `TFLuna` struct at the level of the `i2c` module was kept for now,
  but may be removed in the future.


## [0.1.0] - 2025-09-30

[0.1.0]: https://github.com/AnesBenmerzoug/embedded-tfluna/releases/tag/v0.1.0

_This is the very first release of the `embedded-tfluna` crate._

