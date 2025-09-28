//! Types of returned data from TF-Luna.

/// Structure containing major, minor, and revision numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FirmwareVersion {
    /// Major version number
    pub major: u8,
    /// Minor version number
    pub minor: u8,
    /// Revision version number
    pub revision: u8,
}

/// Structure containing the serial number of the device.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SerialNumber(pub [u8; 14]);

/// ASCII signature of the device. 
/// 
/// The TF-Luna's signature is: 'L', 'U', 'N', 'A'
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Signature(pub [u8; 4]);

/// Ranging modes of the device.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RangingMode {
    /// In Continuous ranging mode, the TF-Luna will keep tracking
    /// the distance at a 500hz frequency, but as the configured
    /// output framerate (frequency) is lower (defaults to 100Hz),
    /// the output will be the average.
    Continuous = 0,
    /// In trigger ranging mode, the TF-Luna stops measuring on its own
    /// and will only measure distance when explicitly triggered.
    Trigger = 1,
}

/// Enum containing the different power modes of the TF-Luna
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerMode {
    /// Normal power mode with largest power consumption
    ///
    /// When the power is supplied with 5V, the power consumption is about 350mW.
    Normal,
    /// Power saving mode with second largest power consumption
    PowerSaving,
    /// Ultra-low power mode with lowest power consumption
    UltraLow,
}

/// Structure containing distance, signal strength, temperature, and timestamp.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorReading {
    /// Distance in centimeters
    pub distance: u16,
    /// Signal strength (amplitude in manual) value between 0 and 1000.
    pub signal_strength: u16,
    /// Internal device temperature in °C with 0.01 precision.
    pub temperature: f32,
    /// Clock ticks since device was powered on.
    pub timestamp: u16,
    /// Error code
    pub error: u16,
}
