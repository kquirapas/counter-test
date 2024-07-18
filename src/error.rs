use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive)]
pub enum CounterTestError {
    #[error("Invalid account data length")]
    InvalidAccountDataLength, // 0
}

// allow .into() for Custom Error to ProgramError conversion
impl From<CounterTestError> for ProgramError {
    fn from(e: CounterTestError) -> Self {
        // https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for CounterTestError {
    fn type_of() -> &'static str {
        "CounterTestError"
    }
}

impl PrintProgramError for CounterTestError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}
