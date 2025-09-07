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
    use esp_hal::clock::CpuClock;

    // init function which is called before every test
    #[init]
    fn init() {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
    }

    #[test]
    fn it_works() {
        assert!(true);
    }
}