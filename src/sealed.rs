use core::convert::Infallible;
use core::num::TryFromIntError;

pub trait IntegerDivision {}

pub trait IntoTryFromIntError: core::fmt::Debug {
    fn into(self) -> TryFromIntError;
}

impl IntoTryFromIntError for TryFromIntError {
    fn into(self) -> TryFromIntError {
        self
    }
}

impl IntoTryFromIntError for Infallible {
    fn into(self) -> TryFromIntError {
        match self {}
    }
}
