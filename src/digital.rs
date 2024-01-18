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

    fn len(&self, digit: usize) -> usize {
        self.0.len(digit)
    }

    fn digit(&self, index: usize, digit_index: usize) -> char {
        self.0.digit(index, digit_index)
    }
}

macro_rules! impl_digit_set {
    ($(#$doc:tt)* $name:ident, $type:ty = $digits:expr) => {
        $(#$doc)*
        // When adding a new variant and getting an error here, either
        // temporarily comment this out or add an empty file until the new
        // variant has been added to the previews example.
        #[doc = include_str!(concat!("./previews/",stringify!($name), ".md"))]
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

                fn len(&self, digit: usize) -> usize {
                    DIGITS.len(digit)
                }

                fn digit(&self, index: usize, digit_index: usize) -> char {
                    DIGITS.digit(index, digit_index)
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
    DigitSet<10, true> = DigitSet::new(['\u{0660}', '\u{0661}', '\u{0662}', '\u{0663}', '\u{0664}', '\u{0665}', '\u{0666}', '\u{0667}', '\u{0668}', '\u{0669}'])
);

impl_digit_set!(
    /// Persian numeric digits.
    Persian,
    DigitSet<10, true> = DigitSet::new(['\u{06F0}', '\u{06F1}', '\u{06F2}', '\u{06F3}', '\u{06F4}', '\u{06F5}', '\u{06F6}', '\u{06F7}', '\u{06F8}', '\u{06F9}'])
);

/// Urdu numeric digits.
///
/// The Unicode codepoints for both [`Persian`] and Urdu numeric digits are
/// identical. The glyph selection comes from using different fonts based on the
/// language desired.
pub use Persian as Urdu;

impl_digit_set!(
    /// Bengali numeric digits.
    Bengali,
    DigitSet<10, true> = DigitSet::new(['\u{9E6}','\u{9E7}','\u{9E8}','\u{9E9}','\u{9EA}','\u{9EB}','\u{9EC}','\u{9ED}','\u{9EE}','\u{9EF}'])
);

impl_digit_set!(
    /// Cambodian numeric digits.
    Cambodian,
    DigitSet<10, true> = DigitSet::new(['\u{17E0}','\u{17E1}','\u{17E2}','\u{17E3}','\u{17E4}','\u{17E5}','\u{17E6}','\u{17E7}','\u{17E8}','\u{17E9}'])
);

/// Khmer numeric digits.
///
/// This set utilizes the same unicode code points as [`Cambodian`].
pub use Cambodian as Khmer;

impl_digit_set!(
    /// CJK Han decimal digits.
    CjkDecimal,
    DigitSet<10, true> = DigitSet::new(['\u{3007}','\u{4E00}','\u{4E8C}','\u{4E09}','\u{56DB}','\u{4E94}','\u{516D}','\u{4E03}','\u{516B}','\u{4E5D}'])
);

/// CJK Heavenly Stems symbols.
///
/// This digit collection falls back to [`CjkDecimal`] after the set is
/// enumerated.
#[doc = include_str!("./previews/CjkHeavenlyStem.md")]
pub struct CjkHeavenlyStem;

impl DigitCollection for CjkHeavenlyStem {
    fn has_zero_digit(&self) -> bool {
        true
    }

    fn len(&self, _digit: usize) -> usize {
        10
    }

    fn digit(&self, index: usize, digit_index: usize) -> char {
        if digit_index == 0 {
            [
                '\u{7532}', '\u{4E59}', '\u{4E19}', '\u{4E01}', '\u{620A}', '\u{5DF1}', '\u{5E9A}',
                '\u{8F9B}', '\u{58EC}', '\u{7678}',
            ][index]
        } else {
            CjkDecimal.digit(index, digit_index)
        }
    }
}

/// CJK Earthly Branch symbols.
///
/// This digit collection back to [`CjkDecimal`] after the set is enumerated.
#[doc = include_str!("./previews/CjkEarthlyBranch.md")]
pub struct CjkEarthlyBranch;

impl DigitCollection for CjkEarthlyBranch {
    fn has_zero_digit(&self) -> bool {
        true
    }

    fn len(&self, digit: usize) -> usize {
        if digit == 0 {
            12
        } else {
            10
        }
    }

    fn digit(&self, index: usize, digit_index: usize) -> char {
        if digit_index == 0 {
            [
                '\u{5B50}', '\u{4E11}', '\u{5BC5}', '\u{536F}', '\u{8FB0}', '\u{5DF3}', '\u{5348}',
                '\u{672A}', '\u{7533}', '\u{9149}', '\u{620C}', '\u{4EA5}',
            ][index]
        } else {
            CjkDecimal.digit(index, digit_index)
        }
    }
}

impl_digit_set!(
    /// Devanagari numeric digits.
    Devanagari,
    DigitSet<10, true> = DigitSet::new(['\u{966}','\u{967}','\u{968}','\u{969}','\u{96A}','\u{96B}','\u{96C}','\u{96D}','\u{96E}','\u{96F}'])
);

impl_digit_set!(
    /// Gujarati numeric digits.
    Gujarati,
    DigitSet<10, true> = DigitSet::new(['\u{AE6}','\u{AE7}','\u{AE8}','\u{AE9}','\u{AEA}','\u{AEB}','\u{AEC}','\u{AED}','\u{AEE}','\u{AEF}'])
);

impl_digit_set!(
    /// Gurmukhi numeric digits.
    Gurmukhi,
    DigitSet<10, true> = DigitSet::new(['\u{A66}','\u{A67}','\u{A68}','\u{A69}','\u{A6A}','\u{A6B}','\u{A6C}','\u{A6D}','\u{A6E}','\u{A6F}'])
);

impl_digit_set!(
    /// Kannada numeric digits.
    Kannada,
    DigitSet<10, true> = DigitSet::new(['\u{CE6}','\u{CE7}','\u{CE8}','\u{CE9}','\u{CEA}','\u{CEB}','\u{CEC}','\u{CED}','\u{CEE}','\u{CEF}'])
);

impl_digit_set!(
    /// Lao numeric digits.
    Lao,
    DigitSet<10, true> = DigitSet::new(['\u{ED0}','\u{ED1}','\u{ED2}','\u{ED3}','\u{ED4}','\u{ED5}','\u{ED6}','\u{ED7}','\u{ED8}','\u{ED9}'])
);

impl_digit_set!(
    /// Malayalam numeric digits.
    Malayalam,
    DigitSet<10, true> = DigitSet::new(['\u{D66}','\u{D67}','\u{D68}','\u{D69}','\u{D6A}','\u{D6B}','\u{D6C}','\u{D6D}','\u{D6E}','\u{D6F}'])
);

impl_digit_set!(
    /// Mongolian numeric digits.
    Mongolian,
    DigitSet<10, true> = DigitSet::new(['\u{1810}','\u{1811}','\u{1812}','\u{1813}','\u{1814}','\u{1815}','\u{1816}','\u{1817}','\u{1818}','\u{1819}'])
);

impl_digit_set!(
    /// Myanmar numeric digits.
    Myanmar,
    DigitSet<10, true> = DigitSet::new(['\u{1040}','\u{1041}','\u{1042}','\u{1043}','\u{1044}','\u{1045}','\u{1046}','\u{1047}','\u{1048}','\u{1049}'])
);

impl_digit_set!(
    /// Oriya numeric digits.
    Oriya,
    DigitSet<10, true> = DigitSet::new(['\u{B66}','\u{B67}','\u{B68}','\u{B69}','\u{B6A}','\u{B6B}','\u{B6C}','\u{B6D}','\u{B6E}','\u{B6F}'])
);

impl_digit_set!(
    /// Tamil numeric digits.
    Tamil,
    DigitSet<10, true> = DigitSet::new(['\u{BE6}','\u{BE7}','\u{BE8}','\u{BE9}','\u{BEA}','\u{BEB}','\u{BEC}','\u{BED}','\u{BEE}','\u{BEF}'])
);

impl_digit_set!(
    /// Telugu numeric digits.
    Telugu,
    DigitSet<10, true> = DigitSet::new(['\u{C66}','\u{C67}','\u{C68}','\u{C69}','\u{C6A}','\u{C6B}','\u{C6C}','\u{C6D}','\u{C6E}','\u{C6F}'])
);

impl_digit_set!(
    /// Thai numeric digits.
    Thai,
    DigitSet<10, true> = DigitSet::new(['\u{E50}','\u{E51}','\u{E52}','\u{E53}','\u{E54}','\u{E55}','\u{E56}','\u{E57}','\u{E58}','\u{E59}'])
);

impl_digit_set!(
    /// Tibetan numeric digits.
    Tibetan,
    DigitSet<10, true> = DigitSet::new(['\u{F20}','\u{F21}','\u{F22}','\u{F23}','\u{F24}','\u{F25}','\u{F26}','\u{F27}','\u{F28}','\u{F29}'])
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
    Hiragana,
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
    Katakana,
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

    fn len(&self, digit: usize) -> usize {
        self.a.len(digit) + self.b.len(digit)
    }

    fn digit(&self, index: usize, digit_index: usize) -> char {
        if let Some(index) = index.checked_sub(self.a.len(digit_index)) {
            self.b.digit(index, digit_index)
        } else {
            self.a.digit(index, digit_index)
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

    fn len(&self, _digit_index: usize) -> usize {
        self.1
    }

    fn digit(&self, index: usize, digit_index: usize) -> char {
        self.0.digit(index, digit_index)
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

    fn len(&self, _digit_index: usize) -> usize {
        N
    }

    fn digit(&self, index: usize, _digit_index: usize) -> char {
        self.digits[index]
    }
}

impl<const N: usize> DigitCollection for DigitSet<N, true> {
    fn has_zero_digit(&self) -> bool {
        true
    }

    fn len(&self, _digit_index: usize) -> usize {
        N
    }

    fn digit(&self, index: usize, _digit_index: usize) -> char {
        self.digits[index]
    }
}

impl<T, D> NominalSystem<T> for D
where
    D: DigitCollection,
    T: Nominal + UnsignedInteger,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let mut digit_index = 0;
        let Ok(mut count) = T::try_from(self.len(digit_index)) else {
            return Ok(NominalString::from(
                self.digit(nominal.as_usize(), digit_index),
            ));
        };
        let one = T::from(1_u8);
        let mut formatted = NominalString::new_reverse();

        let mut remaining = nominal;
        let mut first_loop = true;
        while !remaining.is_zero() || first_loop {
            if !self.has_zero_digit() && (!self.zero_based() || !first_loop) {
                if !self.has_zero_digit() && remaining.is_zero() {
                    return Err(Error::NoZeroSymbol);
                }

                remaining -= one;
            }
            first_loop = false;

            formatted
                .try_push_front(self.digit((remaining % count).as_usize(), digit_index))
                .with_nominal(nominal)?;
            remaining /= count;
            digit_index += 1;
            count = match T::try_from(self.len(digit_index)) {
                Ok(count) => count,
                Err(_) => return Err(Error::OutOfBounds(nominal)),
            };
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
    fn zero_based(&self) -> bool {
        true
    }
    /// Returns the number of digits in this collection.
    fn len(&self, digit: usize) -> usize;
    /// Returns the digit at location `index`.
    ///
    /// # Panics
    ///
    /// This function can panic if `index >= self.len()`.
    fn digit(&self, index: usize, digit_index: usize) -> char;

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
