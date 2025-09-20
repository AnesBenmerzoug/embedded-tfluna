/// Default I2c Slave Address of the TF-Luna device
pub const DEFAULT_SLAVE_ADDRESS: u8 = 0x10;

// Registers
/// Distance measurement low byte register - centimeters - Read-only
pub const DISTANCE_REGISTER_ADDRESS: u8 = 0x00;
/// Signal strength measurement low byte register - Read-only
pub const SIGNAL_STRENGTH_REGISTER_ADDRESS: u8 = 0x02;
/// Temperature measurement low byte register - 0.01°C units - Read-only
pub const TEMPERATURE_REGISTER_ADDRESS: u8 = 0x04;
/// Timestamp low byte register - device ticks - Read-only
pub const TIMESTAMP_REGISTER_ADDRESS: u8 = 0x06;
/// Error low byte register - Read-only
pub const ERROR_REGISTER_ADDRESS: u8 = 0x08;
/// Firmware revision number register - first of three version registers - Read-only
pub const FIRMWARE_VERSION_REGISTER_ADDRESS: u8 = 0x0A;
/// Serial number first byte register - 14-byte ASCII code - Read-only
pub const SERIAL_NUMBER_REGISTER_ADDRESS: u8 = 0x10;
/// Ultra low power mode configuration register - Write-only
pub const ULTRA_LOW_POWER_POWER_MODE_REGISTER_ADDRESS: u8 = 0x1F;
/// Save settings command register - Write-only
pub const SAVE_REGISTER_ADDRESS: u8 = 0x20;
/// Shutdown/reboot command register - Write-only
pub const SHUTDOWN_REBOOT_REGISTER_ADDRESS: u8 = 0x21;
/// I2C slave address configuration register - Read/Write
pub const SLAVE_ADDRESS_REGISTER_ADDRESS: u8 = 0x22;
/// Ranging mode configuration register - Read/Write
pub const RANGING_MODE_REGISTER_ADDRESS: u8 = 0x23;
/// Trigger one-shot measurement command register - Write-only
pub const TRIGGER_REGISTER_ADDRESS: u8 = 0x24;
/// Enable/disable device register - Read/Write
pub const ENABLE_REGISTER_ADDRESS: u8 = 0x25;
/// Framerate configuration low byte register - Hz - Read/Write
pub const FRAMERATE_REGISTER_ADDRESS: u8 = 0x26;
/// Low power mode configuration register - Read/Write
pub const POWER_MODE_REGISTER_ADDRESS: u8 = 0x28;
/// Restore factory defaults command register - Write-only
pub const RESTORE_FACTORY_DEFAULTS_REGISTER_ADDRESS: u8 = 0x29;
/// Signal strength threshold low byte register - Read/Write
pub const SIGNAL_STRENGTH_THRESHOLD_REGISTER_ADDRESS: u8 = 0x2A;
/// Dummy distance low byte register - centimeters - Read/Write
pub const DUMMY_DISTANCE_REGISTER_ADDRESS: u8 = 0x2C;
/// Minimum distance low byte register - centimeters - Read/Write
pub const MINIMUM_DISTANCE_REGISTER_ADDRESS: u8 = 0x2E;
/// Maximum distance low byte register - centimeters - Read/Write
pub const MAXIMUM_DISTANCE_REGISTER_ADDRESS: u8 = 0x30;
/// Signature lower byte register - 4-byte ASCII code - Read-only
pub const SIGNATURE_REGISTER_ADDRESS: u8 = 0x3C;

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
/// Value to write for setting ultra-low power mode
pub const ULTRA_LOWER_POWER_MODE_COMMAND_VALUE: u8 = 0x01; 

// Other values
pub const SLAVE_ADDRESS_MINIMUM_VALUE: u8 = 0x08;
pub const SLAVE_ADDRESS_MAXIMUM_VALUE: u8 = 0x77;
