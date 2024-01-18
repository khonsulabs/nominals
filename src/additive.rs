use crate::{Error, Nominal, NominalString, NominalSystem, WithNominal};

/// A set of additive symbols that form a [`NominalSystem`].
pub struct AdditiveSet<const N: usize> {
    symbols: [(&'static str, u128); N],
    zero: Option<&'static str>,
}

impl<const N: usize> AdditiveSet<N> {
    /// Creates a new additive set containing `symbols`.
    ///
    /// `symbols` must be specified in decending value order.
    ///
    /// # Panics
    ///
    /// This function panics if `symbols` is not in descending order.
    #[must_use]
    pub const fn new(symbols: [(&'static str, u128); N]) -> Self {
        let mut i = 0;
        while i + 1 < N {
            assert!(symbols[i].1 > symbols[i + 1].1);
            i += 1;
        }

        let zero = if let Some((zero, 0)) = symbols.last() {
            Some(*zero)
        } else {
            None
        };
        Self { symbols, zero }
    }
}

impl<const N: usize, T> NominalSystem<T> for AdditiveSet<N>
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let mut formatted = NominalString::default();

        let mut remaining = nominal;
        if remaining.is_zero() {
            return if let Some(zero) = self.zero {
                Ok(NominalString::from(zero))
            } else {
                Err(Error::NoZeroSymbol)
            };
        }

        for (symbol, value) in self.symbols {
            if value == 0 {
                break;
            }

            let Ok(value_as_t) = T::try_from(value) else {
                continue;
            };
            while remaining >= value_as_t {
                remaining -= value_as_t;
                formatted.try_push_str(symbol).with_nominal(nominal)?;
            }
        }

        Ok(formatted)
    }
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
#[doc = include_str!("./previews/RomanLower.md")]
pub struct RomanLower;

impl<T> NominalSystem<T> for RomanLower
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
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
            ("n", 0),
        ])
        .try_format_nominal(nominal)
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
#[doc = include_str!("./previews/RomanUpper.md")]
pub struct RomanUpper;

impl<T> NominalSystem<T> for RomanUpper
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
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
            ("N", 0),
        ])
        .try_format_nominal(nominal)
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

/// Uppercase Armenian numbering.
#[doc = include_str!("./previews/ArmenianUpper.md")]
pub struct ArmenianUpper;

impl<T> NominalSystem<T> for ArmenianUpper
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("Ք", 9000),
            ("Փ", 8000),
            ("Ւ", 7000),
            ("Ց", 6000),
            ("Ր", 5000),
            ("Տ", 4000),
            ("Վ", 3000),
            ("Ս", 2000),
            ("Ռ", 1000),
            ("Ջ", 900),
            ("Պ", 800),
            ("Չ", 700),
            ("Ո", 600),
            ("Շ", 500),
            ("Ն", 400),
            ("Յ", 300),
            ("Մ", 200),
            ("Ճ", 100),
            ("Ղ", 90),
            ("Ձ", 80),
            ("Հ", 70),
            ("Կ", 60),
            ("Ծ", 50),
            ("Խ", 40),
            ("Լ", 30),
            ("Ի", 20),
            ("Ժ", 10),
            ("Թ", 9),
            ("Ը", 8),
            ("Է", 7),
            ("Զ", 6),
            ("Ե", 5),
            ("Դ", 4),
            ("Գ", 3),
            ("Բ", 2),
            ("Ա ", 1),
        ])
        .try_format_nominal(nominal)
    }
}

/// Lowercase Armenian numbering.
#[doc = include_str!("./previews/ArmenianLower.md")]
pub struct ArmenianLower;

impl<T> NominalSystem<T> for ArmenianLower
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("ք", 9000),
            ("փ", 8000),
            ("ւ", 7000),
            ("ց", 6000),
            ("ր", 5000),
            ("տ", 4000),
            ("վ", 3000),
            ("ս", 2000),
            ("ռ", 1000),
            ("ջ", 900),
            ("պ", 800),
            ("չ", 700),
            ("ո", 600),
            ("շ", 500),
            ("ն", 400),
            ("յ", 300),
            ("մ", 200),
            ("ճ", 100),
            ("ղ", 90),
            ("ձ", 80),
            ("հ", 70),
            ("կ", 60),
            ("ծ", 50),
            ("խ", 40),
            ("լ", 30),
            ("ի", 20),
            ("ժ", 10),
            ("թ", 9),
            ("ը", 8),
            ("է", 7),
            ("զ", 6),
            ("ե", 5),
            ("դ", 4),
            ("գ", 3),
            ("բ", 2),
            ("ա", 1),
        ])
        .try_format_nominal(nominal)
    }
}

/// Traditional Georgian numbering.
#[doc = include_str!("./previews/Georgian.md")]
pub struct Georgian;

impl<T> NominalSystem<T> for Georgian
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{10F5}", 10000),
            ("\u{10F0}", 9000),
            ("\u{10EF}", 8000),
            ("\u{10F4}", 7000),
            ("\u{10EE}", 6000),
            ("\u{10ED}", 5000),
            ("\u{10EC}", 4000),
            ("\u{10EB}", 3000),
            ("\u{10EA}", 2000),
            ("\u{10E9}", 1000),
            ("\u{10E8}", 900),
            ("\u{10E7}", 800),
            ("\u{10E6}", 700),
            ("\u{10E5}", 600),
            ("\u{10E4}", 500),
            ("\u{10F3}", 400),
            ("\u{10E2}", 300),
            ("\u{10E1}", 200),
            ("\u{10E0}", 100),
            ("\u{10DF}", 90),
            ("\u{10DE}", 80),
            ("\u{10DD}", 70),
            ("\u{10F2}", 60),
            ("\u{10DC}", 50),
            ("\u{10DB}", 40),
            ("\u{10DA}", 30),
            ("\u{10D9}", 20),
            ("\u{10D8}", 10),
            ("\u{10D7}", 9),
            ("\u{10F1}", 8),
            ("\u{10D6}", 7),
            ("\u{10D5}", 6),
            ("\u{10D4}", 5),
            ("\u{10D3}", 4),
            ("\u{10D2}", 3),
            ("\u{10D1}", 2),
            ("\u{10D0}", 1),
        ])
        .try_format_nominal(nominal)
    }
}

/// Formal Japanese Kanji numbering.
#[doc = include_str!("./previews/JapaneseFormal.md")]
pub struct JapaneseFormal;

impl<T> NominalSystem<T> for JapaneseFormal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{4E5D}\u{9621}", 9000),
            ("\u{516B}\u{9621}", 8000),
            ("\u{4E03}\u{9621}", 7000),
            ("\u{516D}\u{9621}", 6000),
            ("\u{4F0D}\u{9621}", 5000),
            ("\u{56DB}\u{9621}", 4000),
            ("\u{53C2}\u{9621}", 3000),
            ("\u{5F10}\u{9621}", 2000),
            ("\u{58F1}\u{9621}", 1000),
            ("\u{4E5D}\u{767E}", 900),
            ("\u{516B}\u{767E}", 800),
            ("\u{4E03}\u{767E}", 700),
            ("\u{516D}\u{767E}", 600),
            ("\u{4F0D}\u{767E}", 500),
            ("\u{56DB}\u{767E}", 400),
            ("\u{53C2}\u{767E}", 300),
            ("\u{5F10}\u{767E}", 200),
            ("\u{58F1}\u{767E}", 100),
            ("\u{4E5D}\u{62FE}", 90),
            ("\u{516B}\u{62FE}", 80),
            ("\u{4E03}\u{62FE}", 70),
            ("\u{516D}\u{62FE}", 60),
            ("\u{4F0D}\u{62FE}", 50),
            ("\u{56DB}\u{62FE}", 40),
            ("\u{53C2}\u{62FE}", 30),
            ("\u{5F10}\u{62FE}", 20),
            ("\u{58F1}\u{62FE}", 10),
            ("\u{4E5D}", 9),
            ("\u{516B}", 8),
            ("\u{4E03}", 7),
            ("\u{516D}", 6),
            ("\u{4F0D}", 5),
            ("\u{56DB}", 4),
            ("\u{53C2}", 3),
            ("\u{5F10}", 2),
            ("\u{58F1}", 1),
            ("\u{96F6}", 0),
        ])
        .try_format_nominal(nominal)
    }
}

/// Informal Japanese Kanji numbering.
#[doc = include_str!("./previews/JapaneseInformal.md")]
pub struct JapaneseInformal;

impl<T> NominalSystem<T> for JapaneseInformal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{4E5D}\u{5343}", 9000),
            ("\u{516B}\u{5343}", 8000),
            ("\u{4E03}\u{5343}", 7000),
            ("\u{516D}\u{5343}", 6000),
            ("\u{4E94}\u{5343}", 5000),
            ("\u{56DB}\u{5343}", 4000),
            ("\u{4E09}\u{5343}", 3000),
            ("\u{4E8C}\u{5343}", 2000),
            ("\u{5343}", 1000),
            ("\u{4E5D}\u{767E}", 900),
            ("\u{516B}\u{767E}", 800),
            ("\u{4E03}\u{767E}", 700),
            ("\u{516D}\u{767E}", 600),
            ("\u{4E94}\u{767E}", 500),
            ("\u{56DB}\u{767E}", 400),
            ("\u{4E09}\u{767E}", 300),
            ("\u{4E8C}\u{767E}", 200),
            ("\u{767E}", 100),
            ("\u{4E5D}\u{5341}", 90),
            ("\u{516B}\u{5341}", 80),
            ("\u{4E03}\u{5341}", 70),
            ("\u{516D}\u{5341}", 60),
            ("\u{4E94}\u{5341}", 50),
            ("\u{56DB}\u{5341}", 40),
            ("\u{4E09}\u{5341}", 30),
            ("\u{4E8C}\u{5341}", 20),
            ("\u{5341}", 10),
            ("\u{4E5D}", 9),
            ("\u{516B}", 8),
            ("\u{4E03}", 7),
            ("\u{516D}", 6),
            ("\u{4E94}", 5),
            ("\u{56DB}", 4),
            ("\u{4E09}", 3),
            ("\u{4E8C}", 2),
            ("\u{4E00}", 1),
            ("\u{3007}", 0),
        ])
        .try_format_nominal(nominal)
    }
}

/// Korean Hangeul numbering.
#[doc = include_str!("./previews/HangeulFormal.md")]
pub struct HangeulFormal;

impl<T> NominalSystem<T> for HangeulFormal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{AD6C}\u{CC9C}", 9000),
            ("\u{D314}\u{CC9C}", 8000),
            ("\u{CE60}\u{CC9C}", 7000),
            ("\u{C721}\u{CC9C}", 6000),
            ("\u{C624}\u{CC9C}", 5000),
            ("\u{C0AC}\u{CC9C}", 4000),
            ("\u{C0BC}\u{CC9C}", 3000),
            ("\u{C774}\u{CC9C}", 2000),
            ("\u{C77C}\u{CC9C}", 1000),
            ("\u{AD6C}\u{BC31}", 900),
            ("\u{D314}\u{BC31}", 800),
            ("\u{CE60}\u{BC31}", 700),
            ("\u{C721}\u{BC31}", 600),
            ("\u{C624}\u{BC31}", 500),
            ("\u{C0AC}\u{BC31}", 400),
            ("\u{C0BC}\u{BC31}", 300),
            ("\u{C774}\u{BC31}", 200),
            ("\u{C77C}\u{BC31}", 100),
            ("\u{AD6C}\u{C2ED}", 90),
            ("\u{D314}\u{C2ED}", 80),
            ("\u{CE60}\u{C2ED}", 70),
            ("\u{C721}\u{C2ED}", 60),
            ("\u{C624}\u{C2ED}", 50),
            ("\u{C0AC}\u{C2ED}", 40),
            ("\u{C0BC}\u{C2ED}", 30),
            ("\u{C774}\u{C2ED}", 20),
            ("\u{C77C}\u{C2ED}", 10),
            ("\u{AD6C}", 9),
            ("\u{D314}", 8),
            ("\u{CE60}", 7),
            ("\u{C721}", 6),
            ("\u{C624}", 5),
            ("\u{C0AC}", 4),
            ("\u{C0BC}", 3),
            ("\u{C774}", 2),
            ("\u{C77C}", 1),
            ("\u{C601}", 0),
        ])
        .try_format_nominal(nominal)
    }
}

/// Informal Korean Hangeul numbering.
#[doc = include_str!("./previews/HangeulInformal.md")]
pub struct HangeulInformal;

impl<T> NominalSystem<T> for HangeulInformal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{4E5D}\u{5343}", 9000),
            ("\u{516B}\u{5343}", 8000),
            ("\u{4E03}\u{5343}", 7000),
            ("\u{516D}\u{5343}", 6000),
            ("\u{4E94}\u{5343}", 5000),
            ("\u{56DB}\u{5343}", 4000),
            ("\u{4E09}\u{5343}", 3000),
            ("\u{4E8C}\u{5343}", 2000),
            ("\u{5343}", 1000),
            ("\u{4E5D}\u{767E}", 900),
            ("\u{516B}\u{767E}", 800),
            ("\u{4E03}\u{767E}", 700),
            ("\u{516D}\u{767E}", 600),
            ("\u{4E94}\u{767E}", 500),
            ("\u{56DB}\u{767E}", 400),
            ("\u{4E09}\u{767E}", 300),
            ("\u{4E8C}\u{767E}", 200),
            ("\u{767E}", 100),
            ("\u{4E5D}\u{5341}", 90),
            ("\u{516B}\u{5341}", 80),
            ("\u{4E03}\u{5341}", 70),
            ("\u{516D}\u{5341}", 60),
            ("\u{4E94}\u{5341}", 50),
            ("\u{56DB}\u{5341}", 40),
            ("\u{4E09}\u{5341}", 30),
            ("\u{4E8C}\u{5341}", 20),
            ("\u{5341}", 10),
            ("\u{4E5D}", 9),
            ("\u{516B}", 8),
            ("\u{4E03}", 7),
            ("\u{516D}", 6),
            ("\u{4E94}", 5),
            ("\u{56DB}", 4),
            ("\u{4E09}", 3),
            ("\u{4E8C}", 2),
            ("\u{4E00}", 1),
            ("\u{96F6}", 0),
        ])
        .try_format_nominal(nominal)
    }
}

/// Formal Korean Hanja numbering.
#[doc = include_str!("./previews/HanjaFormal.md")]
pub struct HanjaFormal;

impl<T> NominalSystem<T> for HanjaFormal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        AdditiveSet::new([
            ("\u{4E5D}\u{4EDF}", 9000),
            ("\u{516B}\u{4EDF}", 8000),
            ("\u{4E03}\u{4EDF}", 7000),
            ("\u{516D}\u{4EDF}", 6000),
            ("\u{4E94}\u{4EDF}", 5000),
            ("\u{56DB}\u{4EDF}", 4000),
            ("\u{53C3}\u{4EDF}", 3000),
            ("\u{8CB3}\u{4EDF}", 2000),
            ("\u{58F9}\u{4EDF}", 1000),
            ("\u{4E5D}\u{767E}", 900),
            ("\u{516B}\u{767E}", 800),
            ("\u{4E03}\u{767E}", 700),
            ("\u{516D}\u{767E}", 600),
            ("\u{4E94}\u{767E}", 500),
            ("\u{56DB}\u{767E}", 400),
            ("\u{53C3}\u{767E}", 300),
            ("\u{8CB3}\u{767E}", 200),
            ("\u{58F9}\u{767E}", 100),
            ("\u{4E5D}\u{62FE}", 90),
            ("\u{516B}\u{62FE}", 80),
            ("\u{4E03}\u{62FE}", 70),
            ("\u{516D}\u{62FE}", 60),
            ("\u{4E94}\u{62FE}", 50),
            ("\u{56DB}\u{62FE}", 40),
            ("\u{53C3}\u{62FE}", 30),
            ("\u{8CB3}\u{62FE}", 20),
            ("\u{58F9}\u{62FE}", 10),
            ("\u{4E5D}", 9),
            ("\u{516B}", 8),
            ("\u{4E03}", 7),
            ("\u{516D}", 6),
            ("\u{4E94}", 5),
            ("\u{56DB}", 4),
            ("\u{53C3}", 3),
            ("\u{8CB3}", 2),
            ("\u{58F9}", 1),
            ("\u{96F6}", 0),
        ])
        .try_format_nominal(nominal)
    }
}
