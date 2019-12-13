use std::num::ParseIntError;
use std::sync::mpsc;
use super::DWord;

#[derive(Debug, Fail)]
pub enum IntcodeError {
    #[fail(display = "unknown opcode `{}` met", opcode)]
    UnknownOpcode {opcode: DWord},

    #[fail(display = "code parse failed")]
    BadCode(#[fail(cause)] ParseIntError),

    #[fail(display = "wrapped none")]
    NoneError(std::option::NoneError),

    #[fail(display = "VM is halted")]
    Halted,

    #[fail(display = "invalid mode {}", mode)]
    InvalidMode{mode: DWord},

    #[fail(display = "input required")]
    NeedsInput,

    // Legacy stuff cause I can't be hecked to rewrite old days...

    #[fail(display = "input read failed")]
    LegacyInputReadFailed(#[fail(cause)] mpsc::RecvError),

    #[fail(display = "input read timed out")]
    LegacyInputReadTimedout(#[fail(cause)] mpsc::RecvTimeoutError),

    #[fail(display = "output write failed")]
    LegacyOutputWriteFailed(#[fail(cause)] mpsc::SendError<DWord>),
}

impl std::convert::From<mpsc::RecvError> for IntcodeError {
    fn from(x: mpsc::RecvError) -> Self {
        IntcodeError::LegacyInputReadFailed(x)
    }
}

impl std::convert::From<mpsc::RecvTimeoutError> for IntcodeError {
    fn from(x: mpsc::RecvTimeoutError) -> Self {
        IntcodeError::LegacyInputReadTimedout(x)
    }
}

impl std::convert::From<mpsc::SendError<DWord>> for IntcodeError {
    fn from(x: mpsc::SendError<DWord>) -> Self {
        IntcodeError::LegacyOutputWriteFailed(x)
    }
}

impl std::convert::From<ParseIntError> for IntcodeError {
    fn from(x: ParseIntError) -> Self {
        IntcodeError::BadCode(x)
    }
}

impl std::convert::From<std::option::NoneError> for IntcodeError {
    fn from(x: std::option::NoneError) -> Self {
        IntcodeError::NoneError(x)
    }
}
