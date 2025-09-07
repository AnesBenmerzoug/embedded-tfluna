use embedded_hal::i2c::Error as I2CErrorTrait;

#[derive(Clone, Copy, Debug)]
pub enum Error<I2CError: I2CErrorTrait> {
    I2c(I2CError),
    InvalidData,
    InvalidParameter,
    Other,
}

impl<I2CError> From<I2CError> for Error<I2CError>
where
    I2CError: I2CErrorTrait,
{
    fn from(value: I2CError) -> Self {
        Error::I2c(value)
    }
}
