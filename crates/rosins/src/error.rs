//! Rosins error types.

use randomorg::Error as RandomorgError;
use std::{convert::From, env::VarError, fmt};

#[derive(Debug)]
pub enum Error {
    StdEnv(VarError),
    Randomorg(RandomorgError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::StdEnv(var_error) => fmt::Display::fmt(var_error, f),
            Error::Randomorg(randomorg_error) => fmt::Display::fmt(randomorg_error, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::StdEnv(ref var_error) => Some(var_error),
            Error::Randomorg(ref randomorg_error) => Some(randomorg_error),
        }
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error::StdEnv(value)
    }
}

impl From<RandomorgError> for Error {
    fn from(value: RandomorgError) -> Self {
        Error::Randomorg(value)
    }
}
