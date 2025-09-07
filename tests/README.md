# Integration Tests

## Prerequisites

- TF-Luna LiDAR
- ESP32 C3

## Running Tests

```shell
cargo test --test integration --target riscv32imc-unknown-none-elf
```

Or using the defined alias:

```shell
cargo integration-tests
```
