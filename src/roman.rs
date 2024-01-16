use crate::{Error, Nominal, NominalString, NominalSystem, UnsignedInteger, WithNominal};

fn format_roman<T>(
    zero_digit: &str,
    digits: &[(&str, u32); 25],
    nominal: T,
) -> Result<NominalString, Error<T>>
where
    T: UnsignedInteger + TryFrom<u32>,
{
    let mut formatted = NominalString::default();

    let mut remaining = nominal;
    if remaining.is_zero() {
        return Ok(NominalString::from(zero_digit));
    }

    for digit in digits {
        let Ok(value_as_t) = T::try_from(digit.1) else {
            continue;
        };
        while remaining >= value_as_t {
            remaining = remaining - value_as_t;
            formatted.try_push_str(digit.0).with_nominal(nominal)?;
        }
    }

    Ok(formatted)
}

/// Lowercase Roman numerals.
///
/// This encoding utilizes Vinculum notation for numbers 4,000 and above. This
/// notation uses an overline over a repeated set of symbols. A few examples:
///
/// ```rust
/// use nominals::{Nominal, RomanLower};
///
/// assert_eq!(4_000_u32.to_nominal(&RomanLower), "i̅v̅");
/// assert_eq!(4_001_u32.to_nominal(&RomanLower), "i̅v̅i");
/// assert_eq!(2_000_000_u32.to_nominal(&RomanLower), "m̅m̅");
/// ```
pub struct RomanLower;

impl<T> NominalSystem<T> for RomanLower
where
    T: Nominal + UnsignedInteger + TryFrom<u32>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
    <T as TryInto<usize>>::Error: core::fmt::Debug,
{
    fn try_format_nominal(&self, numeric: T) -> Result<NominalString, Error<T>> {
        format_roman(
            "n",
            &[
                ("m̅", 1_000_000),
                ("d̅m̅", 900_000),
                ("d̅", 500_000),
                ("c̅d̅", 400_000),
                ("c̅", 100_000),
                ("l̅c̅", 90_000),
                ("l̅", 50_000),
                ("x̅l̅", 40_000),
                ("x̅", 10_000),
                ("i̅x̅", 9_000),
                ("v̅", 5_000),
                ("i̅v̅", 4_000),
                ("m", 1_000),
                ("cm", 900),
                ("d", 500),
                ("cd", 400),
                ("c", 100),
                ("xc", 90),
                ("l", 50),
                ("xl", 40),
                ("x", 10),
                ("ix", 9),
                ("v", 5),
                ("iv", 4),
                ("i", 1),
            ],
            numeric,
        )
    }
}

/// Uppercase Roman numerals
///
/// This encoding utilizes Vinculum notation for numbers 4,000 and above. This
/// notation uses an overline over a repeated set of symbols. A few examples:
///
/// ```rust
/// use nominals::{Nominal, RomanUpper};
///
/// assert_eq!(4000_u32.to_nominal(&RomanUpper), "I̅V̅");
/// assert_eq!(4_001_u32.to_nominal(&RomanUpper), "I̅V̅I");
/// assert_eq!(2_000_000_u32.to_nominal(&RomanUpper), "M̅M̅");
/// ```
pub struct RomanUpper;

impl<T> NominalSystem<T> for RomanUpper
where
    T: Nominal + UnsignedInteger + TryFrom<u32>,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
    <T as TryInto<usize>>::Error: core::fmt::Debug,
{
    fn try_format_nominal(&self, numeric: T) -> Result<NominalString, Error<T>> {
        format_roman(
            "N",
            &[
                ("M̅", 1_000_000),
                ("D̅M̅", 900_000),
                ("D̅", 500_000),
                ("C̅D̅", 400_000),
                ("C̅", 100_000),
                ("L̅C̅", 90_000),
                ("L̅", 50_000),
                ("X̅L̅", 40_000),
                ("X̅", 10_000),
                ("I̅X̅", 9_000),
                ("V̅", 5_000),
                ("I̅V̅", 4_000),
                ("M", 1_000),
                ("CM", 900),
                ("D", 500),
                ("CD", 400),
                ("C", 100),
                ("XC", 90),
                ("L", 50),
                ("XL", 40),
                ("X", 10),
                ("IX", 9),
                ("V", 5),
                ("IV", 4),
                ("I", 1),
            ],
            numeric,
        )
    }
}

#[test]
fn roman() {
    use crate::Nominal;

    assert_eq!(0_u32.to_nominal(&RomanUpper), "N");
    assert_eq!(1_u32.to_nominal(&RomanUpper), "I");
    assert_eq!(2_u32.to_nominal(&RomanUpper), "II");
    assert_eq!(3_u32.to_nominal(&RomanUpper), "III");
    assert_eq!(4_u32.to_nominal(&RomanUpper), "IV");
    assert_eq!(5_u32.to_nominal(&RomanUpper), "V");
    assert_eq!(0_u32.to_nominal(&RomanLower), "n");
    assert_eq!(1_u32.to_nominal(&RomanLower), "i");
    assert_eq!(2_u32.to_nominal(&RomanLower), "ii");
    assert_eq!(3_u32.to_nominal(&RomanLower), "iii");
    assert_eq!(4_u32.to_nominal(&RomanLower), "iv");
    assert_eq!(5_u32.to_nominal(&RomanLower), "v");
    assert_eq!(4000_u32.to_nominal(&RomanLower), "i̅v̅");
    assert_eq!(2_000_000_u32.to_nominal(&RomanLower), "m̅m̅");
}

#[test]
fn long_test() {
    assert_eq!(
        63_000_000_u32.to_nominal(&RomanLower),
        "m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅m̅"
    );
}
