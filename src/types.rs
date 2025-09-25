/// Structure containing major, minor, and revision numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SerialNumber(pub [u8; 14]);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Signature(pub [u8; 4]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangingMode {
    Continuous = 0,
    Trigger = 1,
}

/// Enum containing the different power modes of the TF-Luna
#[derive(Debug, Clone, Copy, PartialEq)]
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
///
/// - Distance: In centimeters.
/// - Signal Strength: Signal strength value between 0 and 1000.
/// - Temperature: In °C with 0.01 precision.
/// - Timestamp: Device ticks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensorReading {
    pub distance: u16,
    pub signal_strength: u16,
    pub temperature: f32,
    pub timestamp: u16,
    pub error: u16,
}
