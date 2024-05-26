#[derive(Debug)]
pub enum ProcessorError {
    NotYetImplemented,
    FetchError,
    BufferOverflow,
}

#[derive(Debug)]
pub enum SystemBusError {
    InvalidAddress,
    NotYetImplemented,
}
