/// Default I2c Slave Address of the TF-Luna device
pub const DEFAULT_SLAVE_ADDRESS: u8 = 0x10;

// Command values
/// Value to write for saving current settings
pub const SAVE_COMMAND_VALUE: u8 = 1;
/// Value to write for rebooting device
pub const REBOOT_COMMAND_VALUE: u8 = 2;
/// Value to write for restoring factory defaults
pub const RESTORE_FACTORY_DEFAULTS_COMMAND_VALUE: u8 = 1;
/// Value to write for triggering a measurement - Only useful when trigger ranging mode is selected
pub const TRIGGER_COMMAND_VALUE: u8 = 1;
/// Value to write for enabling device measurements
pub const ENABLE_COMMAND_VALUE: u8 = 1;
/// Value to write for disabling device measurements
pub const DISABLE_COMMAND_VALUE: u8 = 0;
/// Value to write for setting normal power mode
pub const NORMAL_POWER_MODE_COMMAND_VALUE: u8 = 0;
/// Value to write for setting power saving power mode
pub const POWER_SAVING_POWER_MODE_COMMAND_VALUE: u8 = 1;
/// Value to write for setting ultra-low power mode
pub const ULTRA_LOWER_POWER_MODE_COMMAND_VALUE: u8 = 1; 

// Other values
pub const SLAVE_ADDRESS_MINIMUM_VALUE: u8 = 0x08;
pub const SLAVE_ADDRESS_MAXIMUM_VALUE: u8 = 0x77;
