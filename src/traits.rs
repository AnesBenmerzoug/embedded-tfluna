#[derive(Clone, Copy, Debug)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
    pub revision: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct SerialNumber(pub [u8; 14]);

pub trait TFLunaSync {
    type Error;

    fn convert_buffer_into_word(&self, buffer: &[u8; 2]) -> Result<u16, Self::Error> {
        let value = buffer[0] as u16 + ((buffer[1] as u16) << 8);
        Ok(value)
    }

    // Set enable bit
    fn enable(&mut self) -> Result<(), Self::Error>;

    // Unset enable bit
    fn disable(&mut self) -> Result<(), Self::Error>;

    fn reboot(&mut self) -> Result<(), Self::Error>;

    fn get_firmware_version(&mut self) -> Result<FirmwareVersion, Self::Error>;

    fn get_serial_number(&mut self) -> Result<SerialNumber, Self::Error>;
}
