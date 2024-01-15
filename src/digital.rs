use crate::{Error, Nominal, NominalString, NominalSystem, UnsignedInteger, WithNominal};

/// An ordered set of characters that can be treated as digits.
///
/// # Const Generics
///
/// - `N`: The number of digits in this set.
/// - `HAS_ZERO`: When true, the first digit in the set represents `0`.
///   - For [`Decimal`], `HAS_ZERO` is true. This produces the ordering when
///     counting from zero goes:
///
///     | n       | 0 | 1 | 2 | ... | 9 | 10 | 11 | 12 | ... |
///     |---------|---|---|---|-----|---|----|----|----|-----|
///     | nominal | 0 | 1 | 2 | ... | 9 | 10 | 11 | 12 | ... |
///
///   - For [`LetterUpper`], if `A` were treated as a `0`, the first symbol that
///     would appear in the "tens" location would be `B`:
///
///
///     | n       | 0 | 1 | 2 | ... | 25 | 26 | 27 | 28 | ... |
///     |---------|---|---|---|-----|----|----|----|----|-----|
///     | nominal | A | B | C | ... |  Z | BA | BB | BC | ... |
///
///     This is not the expected behavior, so [`LetterUpper`] utilizes
///     [`DigitSet::zeroless`] to create a set with `HAS_ZERO` set to false.
///     This produces the order:
///
///     | n       | 0 | 1 | 2 | ... | 25 | 26 | 27 | 28 | ... |
///     |---------|---|---|---|-----|----|----|----|----|-----|
///     | nominal | A | B | C | ... |  Z | AA | AB | AC | ... |
///
///     If 1-based counting is desired, [`DigitSet::one_based`] returns an
///     updated set that returns [`Error::NoZeroSymbol`] if asked to format `0`.
///     It produces this order:
///
///
///     | n       | 0   | 1 | 2 | 3 | ... | 26 | 27 | 28 | 29 | ... |
///     |---------|-----|---|---|---|-----|----|----|----|----|-----|
///     | nominal | err | A | B | C | ... |  Z | AA | AB | AC | ... |
pub struct DigitSet<const N: usize, const HAS_ZERO: bool> {
    digits: [char; N],
}

impl<const N: usize> DigitSet<N, true> {
    /// Returns a digit set whose first digit represents the `0` digit.
    #[must_use]
    pub const fn new(digits: [char; N]) -> Self {
        Self { digits }
    }
}

impl<const N: usize> DigitSet<N, false> {
    /// Returns a digit set that does not have a symbol representing a `0`
    /// digit.
    #[must_use]
    pub const fn zeroless(digits: [char; N]) -> Self {
        Self { digits }
    }
}

/// Begins counting at 1 when formatting a nominal identifier.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct OneBased<T>(T);

impl<T> OneBased<T> {
    /// Returns `digits`, adjusted to start counting at 1.
    pub const fn new(digits: T) -> Self {
        Self(digits)
    }
}

impl<T> DigitCollection for OneBased<T>
where
    T: DigitCollection,
{
    fn has_zero_digit(&self) -> bool {
        self.0.has_zero_digit()
    }

    fn zero_based(&self) -> bool {
        false
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn digit(&self, index: usize) -> char {
        self.0.digit(index)
    }
}

macro_rules! impl_digit_set {
    ($(#$doc:tt)? $name:ident, $type:ty = $digits:expr) => {
        $(#$doc)?
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub struct $name;

        const _: () = {
            static DIGITS: $type = $digits;

            impl DigitCollection for $name {
                fn has_zero_digit(&self) -> bool {
                    DIGITS.has_zero_digit()
                }

                fn zero_based(&self) -> bool {
                    DIGITS.zero_based()
                }

                fn len(&self) -> usize {
                    DIGITS.len()
                }

                fn digit(&self, index: usize) -> char {
                    DIGITS.digit(index)
                }
            }
        };
    };
}

impl_digit_set!(
    /// Western ASCII numeric digits.
    Decimal,
    DigitSet<10, true> = DigitSet::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
);

impl_digit_set!(
    /// Eastern Arabic numeric digits.
    EasternArabic,
    DigitSet<10, true> = DigitSet::new(['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'])
);

impl_digit_set!(
    /// Persian numeric digits.
    Persian,
    DigitSet<10, true> = DigitSet::new(['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'])
);

impl_digit_set!(
    /// Urdu numeric digits.
    Urdu,
    DigitSet<10, true> = DigitSet::new(['۰', '۱', '۲', '۴', '۴', '۵', '۶', '۷', '۸', '۹'])
);

impl_digit_set!(
    /// ASCII uppercase characters.
    LetterUpper,
    DigitSet<26, false> = DigitSet::zeroless([
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ])
);

impl_digit_set!(
    /// ASCII lowercase characters.
    LetterLower,
    DigitSet<26, false> = DigitSet::zeroless([
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
);

impl_digit_set!(
    /// Greek uppercase alphabet.
    GreekUpper,
    DigitSet<24, false> = DigitSet::zeroless([
        'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ',
        'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
    ])
);

impl_digit_set!(
    /// Greek lowercase alphabet.
    GreekLower,
    DigitSet<24, false> = DigitSet::zeroless([
        'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ',
        'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
    ])
);

impl_digit_set!(
    /// Japanese Hiaragana Aiueo alphabet.
    HiraganaAiueo,
    DigitSet<46, false> = DigitSet::zeroless([
        'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ',
        'た', 'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ',
        'ま', 'み', 'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を',
        'ん',
    ])
);

impl_digit_set!(
    /// Japanese Hiaragana Iroha alphabet.
    HiraganaIroha,
    DigitSet<47, false> = DigitSet::zeroless([
        'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る', 'を', 'わ', 'か', 'よ',
        'た', 'れ', 'そ', 'つ', 'ね', 'な', 'ら', 'む', 'う', 'ゐ', 'の', 'お', 'く', 'や', 'ま',
        'け', 'ふ', 'こ', 'え', 'て', 'あ', 'さ', 'き', 'ゆ', 'め', 'み', 'し', 'ゑ', 'ひ', 'も',
        'せ', 'す',
    ])
);

impl_digit_set!(
    /// Japanese Katakana Aiueo alphabet.
    KatakanaAiueo,
    DigitSet<46, false> = DigitSet::zeroless([
        'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ',
        'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
        'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ',
        'ン',
    ])
);

impl_digit_set!(
    /// Japanese Katakana Iroha alphabet.
    KatakanaIroha,
    DigitSet<47, false> = DigitSet::zeroless([
        'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル', 'ヲ', 'ワ', 'カ', 'ヨ',
        'タ', 'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ', 'ム', 'ウ', 'ヰ', 'ノ', 'オ', 'ク', 'ヤ', 'マ',
        'ケ', 'フ', 'コ', 'エ', 'テ', 'ア', 'サ', 'キ', 'ユ', 'メ', 'ミ', 'シ', 'ヱ', 'ヒ', 'モ',
        'セ', 'ス',
    ])
);

impl_digit_set!(
    /// Korean Hangul Jamo alphabet.
    HangeulJamo,
    DigitSet<14, false> = DigitSet::zeroless([
        'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ])
);

impl_digit_set!(
    /// Korean Hangul alphabet as pronounced.
    HangeulSyllable,
    DigitSet<14, false> = DigitSet::zeroless([
        '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타', '파', '하',
    ])
);

/// A combination of two [`DigitCollection`] implementations.
///
/// Digits from `A` will be selected before digits from `B`.
pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<A, B> Chain<A, B> {
    /// Returns a new combined set of digits.
    pub const fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> DigitCollection for Chain<A, B>
where
    A: DigitCollection,
    B: DigitCollection,
{
    fn has_zero_digit(&self) -> bool {
        self.a.has_zero_digit()
    }

    fn zero_based(&self) -> bool {
        self.a.zero_based()
    }

    fn len(&self) -> usize {
        self.a.len() + self.b.len()
    }

    fn digit(&self, index: usize) -> char {
        if let Some(index) = index.checked_sub(self.a.len()) {
            self.b.digit(index)
        } else {
            self.a.digit(index)
        }
    }
}

/// Restricts a set of digits to a specific length.
pub struct Restrict<T>(T, usize);

impl<T> Restrict<T> {
    /// Returns a new type that restricts `collection` to `count` digits.
    pub const fn new(count: usize, collection: T) -> Self {
        Self(collection, count)
    }
}

impl<T> DigitCollection for Restrict<T>
where
    T: DigitCollection,
{
    fn has_zero_digit(&self) -> bool {
        self.0.has_zero_digit()
    }

    fn zero_based(&self) -> bool {
        self.0.zero_based()
    }

    fn len(&self) -> usize {
        self.1
    }

    fn digit(&self, index: usize) -> char {
        self.0.digit(index)
    }
}

impl_digit_set!(
    /// Hexadecimal uppercase ascii characters.
    HexUpper,
    Restrict<Chain<Decimal, LetterUpper>> = Restrict::new(16, Chain::new(Decimal, LetterUpper))
);

impl_digit_set!(
    /// Hexadecimal lower ascii characters.
    HexLower,
    Restrict<Chain<Decimal, LetterLower>> = Restrict::new(16, Chain::new(Decimal, LetterLower))
);

#[test]
fn hex() {
    assert_eq!(HexUpper.format_nominal(15_u32), "F");
    assert_eq!(HexLower.format_nominal(0xfeed_d0d0_u32), "feedd0d0");
}

impl<const N: usize> DigitCollection for DigitSet<N, false> {
    fn has_zero_digit(&self) -> bool {
        false
    }

    fn zero_based(&self) -> bool {
        true
    }

    fn len(&self) -> usize {
        N
    }

    fn digit(&self, index: usize) -> char {
        self.digits[index]
    }
}

impl<const N: usize> DigitCollection for DigitSet<N, true> {
    fn has_zero_digit(&self) -> bool {
        true
    }

    fn zero_based(&self) -> bool {
        true
    }

    fn len(&self) -> usize {
        N
    }

    fn digit(&self, index: usize) -> char {
        self.digits[index]
    }
}

impl<T, D> NominalSystem<T> for D
where
    D: DigitCollection,
    T: Nominal + UnsignedInteger,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
    <T as TryInto<usize>>::Error: core::fmt::Debug,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let Ok(count) = T::try_from(self.len()) else {
            return Ok(NominalString::from(
                self.digit((nominal).try_into().expect("numeric < len")),
            ));
        };
        let one = T::from(1_u8);
        let mut formatted = NominalString::new();

        let mut remaining = nominal;
        let mut first_loop = true;
        while !remaining.is_zero() || first_loop {
            if !self.has_zero_digit() && (!self.zero_based() || !first_loop) {
                if !self.has_zero_digit() && remaining.is_zero() {
                    return Err(Error::NoZeroSymbol);
                }

                remaining = remaining - one;
            }
            first_loop = false;

            formatted
                .try_push_front(
                    self.digit((remaining % count).try_into().expect("count <= usize::MAX")),
                )
                .with_nominal(nominal)?;
            remaining = remaining / count;
        }

        Ok(formatted)
    }
}

/// An ordered collection of digits that can be used as a [`NominalSystem`].
#[allow(clippy::len_without_is_empty)]
pub trait DigitCollection {
    /// Returns true if this collection has a symbol representing `0` at index
    /// 0.
    fn has_zero_digit(&self) -> bool;
    /// Returns true if this collection should start counting at 1 instead of 0.
    /// This function is only called if `has_zero_digit()` returns false.
    fn zero_based(&self) -> bool;
    /// Returns the number of digits in this collection.
    fn len(&self) -> usize;
    /// Returns the digit at location `index`.
    ///
    /// # Panics
    ///
    /// This function can panic if `index >= self.len()`.
    fn digit(&self, index: usize) -> char;

    /// Chains `self` and `other` into a single [`DigitCollection`].
    fn and<Other>(self, other: Other) -> Chain<Self, Other>
    where
        Self: Sized,
        Other: DigitCollection,
    {
        Chain::new(self, other)
    }

    /// Returns this collection that indicates counting should start at 1
    /// instead of 0.
    ///
    /// This has no effect if the collection has a zero digit.
    fn one_based(self) -> OneBased<Self>
    where
        Self: Sized,
    {
        OneBased::new(self)
    }
}

#[test]
fn basic_digits() {
    assert_eq!(Decimal.format_nominal(0_u8), "0");
    assert_eq!(Decimal.format_nominal(1_u8), "1");
    assert_eq!(Decimal.format_nominal(12_u8), "12");
    assert_eq!(LetterLower.format_nominal(0_u8), "a");
    assert_eq!(LetterUpper.format_nominal(26_u8), "AA");
    assert_eq!(
        LetterLower.one_based().try_format_nominal(0_u8),
        Err(Error::NoZeroSymbol)
    );
    assert_eq!(LetterLower.one_based().format_nominal(1_u8), "a");
    assert_eq!(LetterUpper.one_based().format_nominal(26_u8), "Z");
    assert_eq!(LetterUpper.one_based().format_nominal(27_u8), "AA");

    assert_eq!(core::mem::size_of::<NominalString>(), 64);
}
// #[test]
// fn basic_digits() {
//     assert_eq!(0_u8.formatted_with(&ARABIC), "0");
//     assert_eq!(1_u8.formatted_with(&ARABIC), "1");
//     assert_eq!(12_u8.formatted_with(&ARABIC), "12");
//     assert_eq!(0_u8.formatted_with(&LETTER_LOWER), "a");
//     assert_eq!(26_u8.formatted_with(&LETTER_UPPER), "AA");
//     assert_eq!(
//         0_u8.try_formatted_with(&LETTER_LOWER.one_based()),
//         Err(NoZeroSymbol)
//     );
//     assert_eq!(1_u8.formatted_with(&LETTER_LOWER.one_based()), "a");
//     assert_eq!(26_u8.formatted_with(&LETTER_UPPER.one_based()), "Z");
//     assert_eq!(27_u8.formatted_with(&LETTER_UPPER.one_based()), "AA");
// }
