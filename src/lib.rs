#![no_std]

extern crate alloc;

#[cfg(feature = "chinese")]
mod chinese;
mod sealed;

use alloc::string::String;
use core::convert::Infallible;
use core::fmt::Debug;
use core::ops::{Div, Rem, Sub};

#[cfg(feature = "chinese")]
pub use chinese::Chinese;
use ecow::EcoString;
use sealed::IntoTryFromIntError;

pub trait Numbering<T> {
    type Error: Debug;

    fn format_number(&self, numeric: T) -> EcoString {
        self.try_format_number(numeric).expect("unable to format")
    }

    fn try_format_number(&self, numeric: T) -> Result<EcoString, Self::Error>;
}

pub trait Numeric: Sized {
    fn formatted_with<N>(self, numbering: &N) -> EcoString
    where
        N: Numbering<Self>,
    {
        numbering.format_number(self)
    }

    fn try_formatted_with<N>(self, numbering: &N) -> Result<EcoString, N::Error>
    where
        N: Numbering<Self>,
    {
        numbering.try_format_number(self)
    }
}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}

pub struct Digits<const N: usize, const HAS_ZERO: bool> {
    digits: [char; N],
    zero_based: bool,
}

impl<const N: usize> Digits<N, true> {
    pub const fn new(digits: [char; N]) -> Self {
        Self {
            digits,
            zero_based: true,
        }
    }
}

impl<const N: usize> Digits<N, false> {
    pub const fn zeroless(digits: [char; N]) -> Self {
        Self {
            digits,
            zero_based: true,
        }
    }

    pub const fn one_based(mut self) -> Self {
        self.zero_based = false;
        self
    }
}

pub const ARABIC: Digits<10, true> =
    Digits::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

pub const LETTER_LOWER: Digits<26, false> = Digits::zeroless([
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

pub const LETTER_UPPER: Digits<26, false> = Digits::zeroless([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
]);

pub const HIRAGANA_AIUEO: Digits<46, false> = Digits::zeroless([
    'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'た',
    'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね', 'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み',
    'む', 'め', 'も', 'や', 'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん',
]);

pub const HIRAGANA_IROHA: Digits<47, false> = Digits::zeroless([
    'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る', 'を', 'わ', 'か', 'よ', 'た',
    'れ', 'そ', 'つ', 'ね', 'な', 'ら', 'む', 'う', 'ゐ', 'の', 'お', 'く', 'や', 'ま', 'け', 'ふ',
    'こ', 'え', 'て', 'あ', 'さ', 'き', 'ゆ', 'め', 'み', 'し', 'ゑ', 'ひ', 'も', 'せ', 'す',
]);

pub const KATAKANA_AIUEO: Digits<46, false> = Digits::zeroless([
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
    'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
]);
pub const KATAKANA_IROHA: Digits<47, false> = Digits::zeroless([
    'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル', 'ヲ', 'ワ', 'カ', 'ヨ', 'タ',
    'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ', 'ム', 'ウ', 'ヰ', 'ノ', 'オ', 'ク', 'ヤ', 'マ', 'ケ', 'フ',
    'コ', 'エ', 'テ', 'ア', 'サ', 'キ', 'ユ', 'メ', 'ミ', 'シ', 'ヱ', 'ヒ', 'モ', 'セ', 'ス',
]);

pub const KOREAN_JAMO: Digits<14, false> = Digits::zeroless([
    'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
]);

pub const KOREAN_SYLLABLE: Digits<14, false> = Digits::zeroless([
    '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타', '파', '하',
]);

pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<A, B> Chain<A, B> {
    pub const fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> DigitCollection for Chain<A, B>
where
    A: DigitCollection,
    B: DigitCollection,
{
    type ZeroError = A::ZeroError;

    fn err_if_no_zero(no_zero: bool) -> Result<(), Self::ZeroError> {
        A::err_if_no_zero(no_zero)
    }

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

pub struct Restrict<T>(T, usize);

impl<T> Restrict<T> {
    pub const fn new(count: usize, collection: T) -> Self {
        Self(collection, count)
    }
}

impl<T> DigitCollection for Restrict<T>
where
    T: DigitCollection,
{
    type ZeroError = T::ZeroError;

    fn err_if_no_zero(no_zero: bool) -> Result<(), Self::ZeroError> {
        T::err_if_no_zero(no_zero)
    }

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

pub const HEX_UPPER: Restrict<Chain<Digits<10, true>, Digits<26, false>>> =
    Restrict::new(16, Chain::new(ARABIC, LETTER_UPPER));
pub const HEX_LOWER: Restrict<Chain<Digits<10, true>, Digits<26, false>>> =
    Restrict::new(16, Chain::new(ARABIC, LETTER_LOWER));

#[test]
fn hex() {
    assert_eq!(15_u32.formatted_with(&HEX_UPPER), "F");
    assert_eq!(0xfeedd0d0_u32.formatted_with(&HEX_LOWER), "feedd0d0");
}

#[allow(clippy::len_without_is_empty)]
pub trait DigitCollection {
    type ZeroError: Debug;

    fn err_if_no_zero(_no_zero: bool) -> Result<(), Self::ZeroError>;
    fn has_zero_digit(&self) -> bool;
    fn zero_based(&self) -> bool;
    fn len(&self) -> usize;
    fn digit(&self, index: usize) -> char;
}

impl<const N: usize> DigitCollection for Digits<N, false> {
    type ZeroError = NoZeroSymbol;

    fn err_if_no_zero(no_zero: bool) -> Result<(), Self::ZeroError> {
        if no_zero {
            Err(NoZeroSymbol)
        } else {
            Ok(())
        }
    }

    fn has_zero_digit(&self) -> bool {
        false
    }

    fn zero_based(&self) -> bool {
        self.zero_based
    }

    fn len(&self) -> usize {
        N
    }

    fn digit(&self, index: usize) -> char {
        self.digits[index]
    }
}

impl<const N: usize> DigitCollection for Digits<N, true> {
    type ZeroError = Infallible;

    fn err_if_no_zero(_no_zero: bool) -> Result<(), Self::ZeroError> {
        Ok(())
    }

    fn has_zero_digit(&self) -> bool {
        true
    }

    fn zero_based(&self) -> bool {
        self.zero_based
    }

    fn len(&self) -> usize {
        N
    }

    fn digit(&self, index: usize) -> char {
        self.digits[index]
    }
}

impl<T, D> Numbering<T> for D
where
    D: DigitCollection,
    T: PositiveInteger + From<u8> + TryFrom<usize> + TryInto<usize>,
    <T as TryFrom<usize>>::Error: IntoTryFromIntError,
    <T as TryInto<usize>>::Error: IntoTryFromIntError,
{
    type Error = D::ZeroError;

    fn try_format_number(&self, numeric: T) -> Result<EcoString, Self::Error> {
        let Ok(count) = T::try_from(self.len()) else {
            return Ok(EcoString::from(
                self.digit((numeric).try_into().expect("numeric < len")),
            ));
        };
        let one = T::from(1_u8);
        let mut formatted = CharArray::new();

        let mut remaining = numeric;
        let mut first_loop = true;
        while remaining > T::ZERO || first_loop {
            if !self.has_zero_digit() && (!self.zero_based() || !first_loop) {
                D::err_if_no_zero(remaining == T::ZERO)?;

                remaining = remaining - one;
            }
            first_loop = false;

            formatted.push_front(
                self.digit((remaining % count).try_into().expect("count <= usize::MAX")),
            );
            remaining = remaining / count;
        }

        Ok(formatted.into())
    }
}

#[test]
fn basic_digits() {
    assert_eq!(0_u8.formatted_with(&ARABIC), "0");
    assert_eq!(1_u8.formatted_with(&ARABIC), "1");
    assert_eq!(12_u8.formatted_with(&ARABIC), "12");
    assert_eq!(0_u8.formatted_with(&LETTER_LOWER), "a");
    assert_eq!(26_u8.formatted_with(&LETTER_UPPER), "AA");
    assert_eq!(
        0_u8.try_formatted_with(&LETTER_LOWER.one_based()),
        Err(NoZeroSymbol)
    );
    assert_eq!(1_u8.formatted_with(&LETTER_LOWER.one_based()), "a");
    assert_eq!(26_u8.formatted_with(&LETTER_UPPER.one_based()), "Z");
    assert_eq!(27_u8.formatted_with(&LETTER_UPPER.one_based()), "AA");
}

enum CharArray {
    Inline(InlineString),
    Heap(String),
}

struct InlineString {
    length: usize,
    bytes: [u8; CharArray::INLINE_SIZE],
}

impl InlineString {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.length]
    }

    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl CharArray {
    const INLINE_SIZE: usize = 55;

    fn new() -> CharArray {
        CharArray::Inline(InlineString {
            length: 0,
            bytes: [0; 55],
        })
    }

    fn push_front(&mut self, ch: char) {
        match self {
            CharArray::Inline(inline) => {
                let char_len = ch.len_utf8();
                let new_length = inline.length + char_len;
                if new_length <= Self::INLINE_SIZE {
                    inline.bytes.copy_within(0..inline.length, char_len);
                    ch.encode_utf8(&mut inline.bytes);
                    inline.length = new_length;
                } else {
                    let mut string = String::with_capacity(new_length);
                    string.push(ch);
                    string.push_str(inline.as_str());
                    *self = CharArray::Heap(string);
                }
            }
            CharArray::Heap(s) => s.insert(0, ch),
        }
    }
}

impl From<CharArray> for EcoString {
    fn from(value: CharArray) -> Self {
        match value {
            CharArray::Inline(inline) => Self::from(inline.as_str()),
            CharArray::Heap(s) => Self::from(s),
        }
    }
}

pub trait PositiveInteger:
    Ord
    + Sub<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Sized
    + sealed::IntegerDivision
{
    const ZERO: Self;
}

macro_rules! impl_positive_integer {
    ($type:ident) => {
        impl sealed::IntegerDivision for $type {}
        impl PositiveInteger for $type {
            const ZERO: Self = 0;
        }
    };
}

impl_positive_integer!(u8);
impl_positive_integer!(u16);
impl_positive_integer!(u32);
impl_positive_integer!(u64);
impl_positive_integer!(u128);
impl_positive_integer!(usize);

struct CustomDigit {
    text: &'static str,
    value: u32,
}

impl CustomDigit {
    fn new(text: &'static str, value: u32) -> Self {
        Self { text, value }
    }
}

pub struct Roman;

pub struct RomanLowercase;

fn format_roman<T>(zero_digit: &str, digits: &[CustomDigit; 25], numeric: T) -> EcoString
where
    T: PositiveInteger + TryFrom<u32>,
{
    let mut formatted = EcoString::default();

    let mut remaining = numeric;
    if remaining == T::ZERO {
        return EcoString::from(zero_digit);
    }

    for digit in digits {
        let Ok(value_as_t) = T::try_from(digit.value) else {
            continue;
        };
        while remaining >= value_as_t {
            remaining = remaining - value_as_t;
            formatted.push_str(digit.text);
        }
    }

    formatted
}

impl<T> Numbering<T> for RomanLowercase
where
    T: PositiveInteger + TryFrom<u32>,
{
    type Error = Infallible;

    fn try_format_number(&self, numeric: T) -> Result<EcoString, Self::Error> {
        Ok(format_roman(
            "n",
            &[
                CustomDigit::new("m̅", 1000000),
                CustomDigit::new("d̅m̅", 900000),
                CustomDigit::new("d̅", 500000),
                CustomDigit::new("c̅d̅", 400000),
                CustomDigit::new("c̅", 100000),
                CustomDigit::new("l̅c̅", 90000),
                CustomDigit::new("l̅", 50000),
                CustomDigit::new("x̅l̅", 40000),
                CustomDigit::new("x̅", 10000),
                CustomDigit::new("i̅x̅", 9000),
                CustomDigit::new("v̅", 5000),
                CustomDigit::new("i̅v̅", 4000),
                CustomDigit::new("m", 1000),
                CustomDigit::new("cm", 900),
                CustomDigit::new("d", 500),
                CustomDigit::new("cd", 400),
                CustomDigit::new("c", 100),
                CustomDigit::new("xc", 90),
                CustomDigit::new("l", 50),
                CustomDigit::new("xl", 40),
                CustomDigit::new("x", 10),
                CustomDigit::new("ix", 9),
                CustomDigit::new("v", 5),
                CustomDigit::new("iv", 4),
                CustomDigit::new("i", 1),
            ],
            numeric,
        ))
    }
}

impl<T> Numbering<T> for Roman
where
    T: PositiveInteger + TryFrom<u32>,
    <T as TryFrom<u32>>::Error: IntoTryFromIntError,
{
    type Error = T::Error;

    fn try_format_number(&self, numeric: T) -> Result<EcoString, Self::Error> {
        Ok(format_roman(
            "N",
            &[
                CustomDigit::new("M̅", 1000000),
                CustomDigit::new("D̅M̅", 900000),
                CustomDigit::new("D̅", 500000),
                CustomDigit::new("C̅D̅", 400000),
                CustomDigit::new("C̅", 100000),
                CustomDigit::new("L̅C̅", 90000),
                CustomDigit::new("L̅", 50000),
                CustomDigit::new("X̅L̅", 40000),
                CustomDigit::new("X̅", 10000),
                CustomDigit::new("I̅X̅", 9000),
                CustomDigit::new("V̅", 5000),
                CustomDigit::new("I̅V̅", 4000),
                CustomDigit::new("M", 1000),
                CustomDigit::new("CM", 900),
                CustomDigit::new("D", 500),
                CustomDigit::new("CD", 400),
                CustomDigit::new("C", 100),
                CustomDigit::new("XC", 90),
                CustomDigit::new("L", 50),
                CustomDigit::new("XL", 40),
                CustomDigit::new("X", 10),
                CustomDigit::new("IX", 9),
                CustomDigit::new("V", 5),
                CustomDigit::new("IV", 4),
                CustomDigit::new("I", 1),
            ],
            numeric,
        ))
    }
}

#[test]
fn roman() {
    assert_eq!(0_u32.formatted_with(&Roman), "N");
    assert_eq!(1_u32.formatted_with(&Roman), "I");
    assert_eq!(2_u32.formatted_with(&Roman), "II");
    assert_eq!(3_u32.formatted_with(&Roman), "III");
    assert_eq!(4_u32.formatted_with(&Roman), "IV");
    assert_eq!(5_u32.formatted_with(&Roman), "V");
    assert_eq!(0_u32.formatted_with(&RomanLowercase), "n");
    assert_eq!(1_u32.formatted_with(&RomanLowercase), "i");
    assert_eq!(2_u32.formatted_with(&RomanLowercase), "ii");
    assert_eq!(3_u32.formatted_with(&RomanLowercase), "iii");
    assert_eq!(4_u32.formatted_with(&RomanLowercase), "iv");
    assert_eq!(5_u32.formatted_with(&RomanLowercase), "v");
    assert_eq!(4000_u32.formatted_with(&RomanLowercase), "i̅v̅");
    assert_eq!(2_000_000_u32.formatted_with(&RomanLowercase), "m̅m̅");
}

pub struct Hebrew;

impl<T> Numbering<T> for Hebrew
where
    T: PositiveInteger + TryFrom<u32> + From<u8>,
{
    type Error = NoZeroSymbol;

    fn try_format_number(&self, mut remaining: T) -> Result<EcoString, Self::Error> {
        if remaining == T::ZERO {
            return Err(NoZeroSymbol);
        }

        let fifteen = T::from(15);
        let sixteen = T::from(16);

        let mut formatted = EcoString::default();
        for (symbol, value) in [
            ('ת', 400u32),
            ('ש', 300),
            ('ר', 200),
            ('ק', 100),
            ('צ', 90),
            ('פ', 80),
            ('ע', 70),
            ('ס', 60),
            ('נ', 50),
            ('מ', 40),
            ('ל', 30),
            ('כ', 20),
            ('י', 10),
            ('ט', 9),
            ('ח', 8),
            ('ז', 7),
            ('ו', 6),
            ('ה', 5),
            ('ד', 4),
            ('ג', 3),
            ('ב', 2),
            ('א', 1),
        ] {
            let Ok(value) = T::try_from(value) else {
                continue;
            };

            while remaining >= value {
                if remaining == fifteen {
                    formatted.push_str("ט״ו");
                    break;
                } else if remaining == sixteen {
                    formatted.push_str("ט״ז");
                    break;
                }

                // When a single symbol is used to represent a number, the
                // symbol is wrapped in geresh and gershayim characters to
                // distinguish it from a word.
                let single_symbol = value == remaining && formatted.is_empty();
                if single_symbol {
                    formatted.push('׳');
                }
                remaining = remaining - value;
                formatted.push(symbol);
                if single_symbol {
                    formatted.push('״');
                }
            }
        }

        Ok(formatted)
    }
}

#[test]
fn hebrew() {
    assert_eq!(997_u32.formatted_with(&Hebrew), "תתקצז");
    assert_eq!(1_u32.formatted_with(&Hebrew), "׳א״");
    assert_eq!(0_u32.try_formatted_with(&Hebrew), Err(NoZeroSymbol));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NoZeroSymbol;
