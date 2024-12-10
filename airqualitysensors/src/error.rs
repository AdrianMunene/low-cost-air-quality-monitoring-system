use esp_hal::uart::Error;
use esp_hal::i2c::Error;
use core::fmt;

#[derive(Debug)]
pub enum CommunicationError {
    Uart(UartError),
    I2c(I2cError),
}

impl fmt::Display for CommunicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommunicationError::Uart(e) => write!(f, "UART Error: {:?}"),
            CommunicationError::I2c(e) => write!(f, "I2C Error: {:?}"),
        }
    }
}

impl From<UartError> for CommunicationError {
    fn from(e: UartError) -> Self {
        CommunicationError::Uart(e)
    }
}

impl From<I2cError> for CommunicationError {
    fn from(e: I2cError) -> Self {
        CommunicationError::I2c(e)
    }
}

#[derive(Debug)]
pub enum SensorError {
    InvalidHeader(&'static str),
    ReadFailure(&'static str),
    Communication(CommunicationError),
}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorError::InvalidHeader(msg) => write!(f, "Sensor Error: {}", msg),
            SensorError::ReadFailure(msg) => write!(f, "Sensor Read Failure: {}", msg),
            SensorError::Communication(e) => write!(f, "Communication Error: {}", msg),
        }
    }
}

impl From<CommunicationError> for SensorError {
    fn from(e: CommunicationError) -> Self {
        SensorError::Communication(e)
    }
}