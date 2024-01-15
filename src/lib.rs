#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "chinese")]
mod chinese;
mod hebrew;
mod roman;
mod sealed;
mod stackstring;

/// Systems that operate using ordered sets of digit-like characters.
mod digital;

use core::fmt::Debug;
use core::ops::{Div, Rem, Sub};

#[cfg(feature = "chinese")]
pub use chinese::Chinese;
pub use digital::*;
pub use hebrew::Hebrew;
pub use roman::{RomanLowercase, RomanUpper};
pub use stackstring::NominalString;

/// A system of ordered nominal identifiers.
pub trait NominalSystem<T> {
    /// The error type that this system can produce.
    type Error: Debug;

    /// Formats `nominal` using this system.
    ///
    /// # Panics
    ///
    /// If [`Self::try_format_nominal()`] returns an error, this function will
    /// panic.
    fn format_nominal(&self, nominal: T) -> NominalString {
        self.try_format_nominal(nominal).expect("unable to format")
    }

    /// Tries to format `nominal` using this system.
    ///
    /// # Errors
    ///
    /// Each nominal system can use its own error type. The crate-level error
    /// type is [`Error`], and each variant describes why formatting a nominal
    /// may fail.
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Self::Error>;
}

/// A type that can be formatted with a [`NominalSystem`].
pub trait Nominal: Sized {
    /// Returns `self` formatted as a nominal identifier using `system`.
    ///
    /// # Panics
    ///
    /// If [`N::try_format_nominal()`](NominalSystem::try_format_nominal)
    /// returns an error, this function will panic.
    fn to_nominal<N>(self, system: &N) -> NominalString
    where
        N: NominalSystem<Self>,
    {
        system.format_nominal(self)
    }

    /// Tries to format `self` as a nominal identifier using `system`.
    ///
    /// # Errors
    ///
    /// Each nominal system can use its own error type. The crate-level error
    /// type is [`Error`], and each variant describes why formatting a nominal
    /// may fail.
    fn try_to_nominal<N>(self, system: &N) -> Result<NominalString, N::Error>
    where
        N: NominalSystem<Self>,
    {
        system.try_format_nominal(self)
    }
}

impl Nominal for u8 {}
impl Nominal for u16 {}
impl Nominal for u32 {}
impl Nominal for u64 {}
impl Nominal for u128 {}
impl Nominal for usize {}

/// An unsigned integer type.
pub trait UnsignedInteger:
    Ord
    + Sub<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Sized
    + sealed::IntegerDivision
{
    /// Returns true if `self` is 0.
    fn is_zero(self) -> bool;
}

macro_rules! impl_positive_integer {
    ($type:ident) => {
        impl sealed::IntegerDivision for $type {}
        impl UnsignedInteger for $type {
            fn is_zero(self) -> bool {
                self == 0
            }
        }
    };
}

impl_positive_integer!(u8);
impl_positive_integer!(u16);
impl_positive_integer!(u32);
impl_positive_integer!(u64);
impl_positive_integer!(u128);
impl_positive_integer!(usize);

/// Error types that can arise from formatting nominals in this crate.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// A formatting request for index `0` was made against a nominal system
    /// that has no zero symbol.
    NoZeroSymbol,
    /// The nominal can't be formatted with the available memory.
    OutOfMemory,
}
