/// Default I2c Slave Address of the TF-Luna device
pub const DEFAULT_SLAVE_ADDRESS: u8 = 0x10;

// Registers
/// Distance measurement low byte register (0x00) - centimeters
pub const DISTANCE_REGISTER_ADDRESS: u8 = 0x00;
/// Signal strength measurement low byte register (0x02)
pub const SIGNAL_STRENGTH_REGISTER_ADDRESS: u8 = 0x02;
/// Temperature measurement low byte register (0x04) - 0.01°C units
pub const TEMPERATURE_REGISTER_ADDRESS: u8 = 0x04;
/// Timestamp low byte register (0x06) - device ticks
pub const TIMESTAMP_REGISTER_ADDRESS: u8 = 0x06;
/// Firmware revision number register (0x0A) - first of three version registers
pub const FIRMWARE_VERSION_REGISTER_ADDRESS: u8 = 0x0A;
/// Serial number first byte register (0x10) - 14-byte ASCII code
pub const SERIAL_NUMBER_REGISTER_ADDRESS: u8 = 0x10;
/// Save settings command register (0x20)
pub const SAVE_REGISTER_ADDRESS: u8 = 0x20;
/// Shutdown/reboot command register (0x21)
pub const SHUTDOWN_REBOOT_REGISTER_ADDRESS: u8 = 0x21;
/// I2C slave address configuration register (0x22)
pub const SLAVE_ADDRESS_REGISTER_ADDRESS: u8 = 0x22;
/// Ranging mode configuration register (0x23)
pub const RANGING_MODE_REGISTER_ADDRESS: u8 = 0x23;
/// Trigger measurement command register (0x24)
pub const TRIGGER_REGISTER_ADDRESS: u8 = 0x24;
/// Enable/disable device register (0x25)
pub const ENABLE_REGISTER_ADDRESS: u8 = 0x25;
/// Framerate configuration low byte register (0x26) - Hz
pub const FRAMERATE_REGISTER_ADDRESS: u8 = 0x26;
/// Power mode configuration register (0x28)
pub const POWER_MODE_REGISTER_ADDRESS: u8 = 0x28;
/// Restore factory defaults command register (0x29)
pub const RESTORE_FACTORY_DEFAULTS_REGISTER_ADDRESS: u8 = 0x29;
/// Signal strength threshold low byte register (0x2A)
pub const SIGNAL_STRENGTH_THRESHOLD_REGISTER_ADDRESS: u8 = 0x2A;
/// Dummy distance low byte register (0x2C) - centimeters
pub const DUMMY_DISTANCE_REGISTER_ADDRESS: u8 = 0x2C;
/// Minimum distance low byte register (0x2E) - centimeters
pub const MINIMUM_DISTANCE_REGISTER_ADDRESS: u8 = 0x2E;
/// Maximum distance low byte register (0x30) - centimeters
pub const MAXIMUM_DISTANCE_REGISTER_ADDRESS: u8 = 0x30;

// Command values
/// Value to write for saving current settings
pub const SAVE_COMMAND_VALUE: u8 = 0x01;
/// Value to write for rebooting device
pub const REBOOT_COMMAND_VALUE: u8 = 0x02;
/// Value to write for restoring factory defaults
pub const RESTORE_FACTORY_DEFAULTS_COMMAND_VALUE: u8 = 0x01;
/// Value to write for setting continuous ranging mode
pub const RANGING_MODE_CONTINUOUS_COMMAND_VALUE: u8 = 0x00;
/// Value to write for setting trigger ranging mode
pub const RANGING_MODE_TRIGGER_COMMAND_VALUE: u8 = 0x01;
/// Value to write for triggering a measurement - Only useful when trigger ranging mode is selected
pub const TRIGGER_COMMAND_VALUE: u8 = 0x01;
/// Value to write for enabling device measurements
pub const ENABLE_COMMAND_VALUE: u8 = 0x01;
/// Value to write for disabling device measurements
pub const DISABLE_COMMAND_VALUE: u8 = 0x00;
/// Value to write for setting normal power mode
pub const NORMAL_POWER_MODE_COMMAND_VALUE: u8 = 0x00;
/// Value to write for setting power saving power mode
pub const POWER_SAVING_POWER_MODE_COMMAND_VALUE: u8 = 0x01;

// Other values
pub const SLAVE_ADDRESS_MINIMUM_VALUE: u8 = 0x08;
pub const SLAVE_ADDRESS_MAXIMUM_VALUE: u8 = 0x77;
