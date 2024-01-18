#![doc = include_str!(".crate-docs.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod additive;
mod chinese;
mod hebrew;
mod nominalstring;

mod sealed {
    /// A trait that marks a type as performing integer-style division with its
    /// `Div` and `Rem` implementations.
    pub trait IntegerDivision {}
}

/// Systems that operate using ordered sets of digit-like characters.
mod digital;

use core::fmt::Debug;
use core::ops::{Div, Mul, Rem, Sub};

pub use additive::*;
pub use chinese::*;
pub use digital::*;
pub use hebrew::Hebrew;
pub use nominalstring::{NominalString, OutOfMemoryError};

/// A system of ordered nominal identifiers.
pub trait NominalSystem<T>
where
    T: Nominal,
{
    /// Formats `nominal` using this system.
    fn format_nominal(&self, nominal: T) -> NominalString {
        self.try_format_nominal(nominal).unwrap_or_decimal()
    }

    /// Tries to format `nominal` using this system.
    ///
    /// # Errors
    ///
    /// Each nominal system can use its own error type. The crate-level error
    /// type is [`Error`], and each variant describes why formatting a nominal
    /// may fail.
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>>;
}

#[cfg(feature = "alloc")]
impl<T> NominalSystem<T> for alloc::boxed::Box<dyn NominalSystem<T>>
where
    T: Nominal,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        self.as_ref().try_format_nominal(nominal)
    }
}

#[cfg(feature = "alloc")]
impl<T> NominalSystem<T> for alloc::rc::Rc<dyn NominalSystem<T>>
where
    T: Nominal,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        self.as_ref().try_format_nominal(nominal)
    }
}

#[cfg(feature = "alloc")]
impl<T> NominalSystem<T> for alloc::sync::Arc<dyn NominalSystem<T>>
where
    T: Nominal,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        self.as_ref().try_format_nominal(nominal)
    }
}

#[test]
fn boxing() {
    let system: alloc::boxed::Box<dyn NominalSystem<u32>> = alloc::boxed::Box::new(RomanUpper);
    assert_eq!(1.to_nominal(&system), "I");
    let system: alloc::rc::Rc<dyn NominalSystem<u32>> = alloc::rc::Rc::new(RomanUpper);
    assert_eq!(1.to_nominal(&system), "I");
    let system: alloc::sync::Arc<dyn NominalSystem<u32>> = alloc::sync::Arc::new(RomanUpper);
    assert_eq!(1.to_nominal(&system), "I");
}

/// A type that can be formatted with a [`NominalSystem`].
pub trait Nominal: UnsignedInteger {
    /// Returns `self` formatted as a nominal identifier using `system`.
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
    fn try_to_nominal<N>(self, system: &N) -> Result<NominalString, Error<Self>>
    where
        N: NominalSystem<Self> + ?Sized,
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
    + From<u8>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Sized
    + sealed::IntegerDivision
    + TryFrom<usize>
    + TryInto<usize>
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
#[non_exhaustive]
pub enum Error<T> {
    /// A formatting request for index `0` was made against a nominal system
    /// that has no zero symbol.
    NoZeroSymbol,
    /// This nominal can't be formatted with the available memory.
    OutOfMemory(T),
    /// This number cannot be represented by the nominal system.
    OutOfBounds(T),
}

impl<T> Error<T>
where
    T: Nominal,
{
    /// Converts this error to a nominal string in decimal form.
    ///
    /// - [`Error::NoZeroSymbol`] is returned as 0
    /// - [`Error::OutOfBounds`] and [`Error::OutOfMemory`] will format the
    ///   erroring nominal in [`Decimal`].
    pub fn into_decimal(self) -> NominalString {
        match self {
            Error::NoZeroSymbol => NominalString::from('0'),
            Error::OutOfBounds(nominal) | Error::OutOfMemory(nominal) => {
                Decimal.format_nominal(nominal)
            }
        }
    }
}

/// Unwraps a result with an [`Error`] by formatting the erroring nominal in
/// [`Decimal`].
pub trait UnwrapOrDecimal {
    /// Returns the nominal string or a nominal string formatted using
    /// [`Decimal`].
    fn unwrap_or_decimal(self) -> NominalString;
}

impl<T> UnwrapOrDecimal for Result<NominalString, Error<T>>
where
    T: Nominal,
{
    fn unwrap_or_decimal(self) -> NominalString {
        match self {
            Ok(s) => s,
            Err(err) => err.into_decimal(),
        }
    }
}

/// Converts a result from one error type to an [`Error<N>`].
pub trait WithNominal<R> {
    /// Returns the result with an updated error containing `nominal`.
    ///
    /// # Errors
    ///
    /// Returns an error if `self` is `Err()`.
    fn with_nominal<N>(self, nominal: N) -> Result<R, Error<N>>;
}

impl<R> WithNominal<R> for Result<R, OutOfMemoryError> {
    fn with_nominal<N>(self, nominal: N) -> Result<R, Error<N>> {
        self.map_err(|_| Error::OutOfMemory(nominal))
    }
}
