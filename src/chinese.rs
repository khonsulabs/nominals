use core::array;

use crate::{Error, Nominal, NominalString, NominalSystem, OutOfMemoryError};

const SIMPLIFIED_ORDINARY: [char; 14] = [
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百', '千', '负',
];
const SIMPLIFIED_FINANCIAL: [char; 14] = [
    '零', '壹', '贰', '叁', '肆', '伍', '陆', '柒', '捌', '玖', '拾', '佰', '仟', '负',
];
const TRADITIONAL_ORDINARY: [char; 14] = [
    '零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '百', '千', '負',
];
const TRADITIONAL_FINANCIAL: [char; 14] = [
    '零', '壹', '貳', '參', '肆', '伍', '陸', '柒', '捌', '玖', '拾', '佰', '仟', '負',
];

const TRADITIONAL_LARGE: [char; 11] = [
    '萬', '億', '兆', '京', '垓', '秭', '穰', '溝', '澗', '正', '載',
];
const SIMPLIFIED_LARGE: [char; 11] = [
    '万', '亿', '兆', '京', '垓', '秭', '穰', '沟', '涧', '正', '载',
];

fn format_chinese<T, const FORMAL: bool>(
    characters: &[char; 14],
    large_characters: &[char; 11],
    nominal: T,
    scale: ChineseScale,
) -> Result<NominalString, Error<T>>
where
    T: Nominal + TryFrom<u128>,
{
    if nominal.is_zero() {
        return Ok(NominalString::from(characters[0]));
    }
    if let Ok(ten_thousand) = T::try_from(10_000usize) {
        if nominal >= ten_thousand {
            return scale
                .format::<T, FORMAL>(characters, large_characters, nominal)
                .map_err(|err| match err {
                    ChineseFormatError::OutOfMemory => Error::OutOfMemory(nominal),
                    ChineseFormatError::OutOfBounds => Error::OutOfBounds(nominal),
                });
        }
    }

    format_chinese_inner::<T, FORMAL>(characters, nominal, true, &mut false)
        .map_err(|_| Error::OutOfMemory(nominal))
}

fn format_chinese_inner<T, const FORMAL: bool>(
    characters: &[char; 14],
    nominal: T,
    no_prefix: bool,
    last_char_is_zero: &mut bool,
) -> Result<NominalString, OutOfMemoryError>
where
    T: Nominal,
{
    let one = T::from(1);
    let ten = T::from(10);

    let mut remaining = nominal;
    let ones = remaining % ten;
    remaining = remaining / ten;
    let tens = remaining % ten;
    remaining = remaining / ten;
    let hundreds = remaining % ten;
    let thousands = remaining / ten;

    let mut formatted = NominalString::new();

    if thousands.is_zero() {
        if !no_prefix && !*last_char_is_zero && !hundreds.is_zero() {
            *last_char_is_zero = true;
            formatted.try_push(characters[0])?;
        }
    } else {
        formatted.try_push(characters[thousands.try_into().map_err(|_| unreachable!("< 10"))?])?;
        formatted.try_push(characters[12])?;
        *last_char_is_zero = false;
    }

    if hundreds.is_zero() {
        if (!formatted.is_empty() || !no_prefix) && !*last_char_is_zero && !tens.is_zero() {
            *last_char_is_zero = true;
            formatted.try_push(characters[0])?;
        }
    } else {
        formatted.try_push(characters[hundreds.try_into().map_err(|_| unreachable!("< 10"))?])?;
        formatted.try_push(characters[11])?;
        *last_char_is_zero = false;
    }

    if tens.is_zero() {
        if (!formatted.is_empty() || !no_prefix) && !*last_char_is_zero && !ones.is_zero() {
            formatted.try_push(characters[0])?;
            *last_char_is_zero = true;
        }
    } else {
        let omit_digit =
            !(FORMAL || (!formatted.is_empty() || !no_prefix) || *last_char_is_zero || tens != one);
        if !omit_digit {
            formatted.try_push(characters[tens.try_into().map_err(|_| unreachable!("< 10"))?])?;
        }
        formatted.try_push(characters[10])?;
        *last_char_is_zero = false;
    }

    if !ones.is_zero() {
        formatted.try_push(characters[ones.try_into().map_err(|_| unreachable!("< 10"))?])?;
        *last_char_is_zero = false;
    }

    Ok(formatted)
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct ChineseOptions {
    usage: ChineseUsage,
    scale: ChineseScale,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
enum ChineseUsage {
    Financial,
    #[default]
    Ordinary,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
enum ChineseScale {
    Short,
    #[default]
    Myriad,
    Mid,
    Long,
}

impl ChineseScale {
    fn format<T, const FORMAL: bool>(
        self,
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        match self {
            ChineseScale::Short => {
                Self::format_short_rank::<T, FORMAL>(10, characters, large_characters, nominal)
            }
            ChineseScale::Myriad => Self::format_scaled::<T, FORMAL, 9>(
                MYRIAD_SCALE,
                characters,
                large_characters,
                nominal,
            ),
            ChineseScale::Mid => Self::format_scaled::<T, FORMAL, 5>(
                MID_SCALE,
                characters,
                large_characters,
                nominal,
            ),
            ChineseScale::Long => Self::format_scaled::<T, FORMAL, 4>(
                LONG_SCALE,
                characters,
                large_characters,
                nominal,
            ),
        }
    }

    fn format_short_rank<T, const FORMAL: bool>(
        mut rank: usize,
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        let Ok(mut scale) = T::try_from(SHORT_SCALE[rank]) else {
            return Self::format_short_rank::<T, FORMAL>(
                rank - 1,
                characters,
                large_characters,
                nominal,
            );
        };

        let mut formatted = NominalString::new();
        let mut last_was_zero = false;
        let mut remaining = nominal;
        while !remaining.is_zero() {
            let rank_value = remaining / scale;
            if rank_value >= T::from(10) {
                return Err(ChineseFormatError::OutOfBounds);
            }

            remaining = remaining % scale;

            if rank_value.is_zero() {
                if !formatted.is_empty() && !last_was_zero {
                    last_was_zero = true;
                    formatted.try_push(characters[0])?;
                }
            } else {
                formatted.try_push(
                    characters[rank_value
                        .try_into()
                        .map_err(|_| ChineseFormatError::OutOfBounds)?],
                )?;
                formatted.try_push(large_characters[rank])?;
                last_was_zero = false;
            }

            if rank == 0 {
                break;
            }
            rank -= 1;
            let Ok(new_scale) = T::try_from(SHORT_SCALE[rank]) else {
                unreachable!("rank + 1 was valid")
            };
            scale = new_scale;
        }

        if !remaining.is_zero() {
            let remaining = format_chinese_inner::<T, FORMAL>(
                characters,
                remaining,
                false,
                &mut last_was_zero,
            )?;
            formatted.try_push_str(&remaining)?;
        }
        Ok(formatted)
    }

    fn format_scaled<T, const FORMAL: bool, const N: usize>(
        scales: [u128; N],
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        let scales: [Option<T>; N] = array::from_fn(|index| T::try_from(scales[index]).ok());
        Self::format_scaled_inner::<T, FORMAL, N>(
            scales,
            characters,
            large_characters,
            nominal,
            true,
            &mut false,
        )
    }

    fn format_scaled_inner<T, const FORMAL: bool, const N: usize>(
        scales: [Option<T>; N],
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
        no_prefix: bool,
        last_char_is_zero: &mut bool,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        Self::format_scaled_rank::<T, FORMAL, N>(
            10,
            scales,
            characters,
            large_characters,
            nominal,
            no_prefix,
            last_char_is_zero,
        )
    }

    fn format_next_scaled_rank<T, const FORMAL: bool, const N: usize>(
        rank: usize,
        scales: [Option<T>; N],
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
        no_prefix: bool,
        last_char_is_zero: &mut bool,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        if let Some(rank) = rank.checked_sub(1) {
            Self::format_scaled_rank::<T, FORMAL, N>(
                rank,
                scales,
                characters,
                large_characters,
                nominal,
                no_prefix,
                last_char_is_zero,
            )
        } else {
            format_chinese_inner::<T, FORMAL>(characters, nominal, no_prefix, last_char_is_zero)
                .map_err(ChineseFormatError::from)
        }
    }

    fn format_scaled_rank<T, const FORMAL: bool, const N: usize>(
        rank: usize,
        scales: [Option<T>; N],
        characters: &[char; 14],
        large_characters: &[char; 11],
        nominal: T,
        no_prefix: bool,
        last_char_is_zero: &mut bool,
    ) -> Result<NominalString, ChineseFormatError>
    where
        T: Nominal + TryFrom<u128>,
    {
        let Some(scale) = scales.get(rank).copied().flatten() else {
            return Self::format_next_scaled_rank::<T, FORMAL, N>(
                rank,
                scales,
                characters,
                large_characters,
                nominal,
                no_prefix,
                last_char_is_zero,
            );
        };
        let remaining = nominal % scale;
        let rank_value = nominal / scale;

        if let Some(previous_scale) = scales.get(rank + 1).copied().flatten() {
            let factor = previous_scale / scale;
            if rank_value >= factor {
                return Err(ChineseFormatError::OutOfBounds);
            }
        } else if rank > 0 {
            let factor = scale / scales[rank - 1].expect("rank is valid");
            if rank_value >= factor {
                return Err(ChineseFormatError::OutOfBounds);
            }
        }
        let mut formatted = if rank_value.is_zero() {
            NominalString::new()
        } else {
            let mut rank_formatted = Self::format_next_scaled_rank::<T, FORMAL, N>(
                rank,
                scales,
                characters,
                large_characters,
                rank_value,
                no_prefix,
                last_char_is_zero,
            )?;
            if !rank_formatted.is_empty() {
                rank_formatted.try_push(large_characters[rank])?;
                *last_char_is_zero = false;
            }
            rank_formatted
        };
        if remaining.is_zero() {
            Ok(formatted)
        } else {
            let remaining = Self::format_next_scaled_rank::<T, FORMAL, N>(
                rank,
                scales,
                characters,
                large_characters,
                remaining,
                formatted.is_empty() && no_prefix,
                last_char_is_zero,
            )?;
            formatted.try_push_str(&remaining)?;
            Ok(formatted)
        }
    }
}

enum ChineseFormatError {
    OutOfBounds,
    OutOfMemory,
}

impl From<OutOfMemoryError> for ChineseFormatError {
    fn from(_value: OutOfMemoryError) -> Self {
        ChineseFormatError::OutOfMemory
    }
}

const SHORT_SCALE: [u128; 11] = [
    10_u128.pow(4),
    10_u128.pow(5),
    10_u128.pow(6),
    10_u128.pow(7),
    10_u128.pow(8),
    10_u128.pow(9),
    10_u128.pow(10),
    10_u128.pow(11),
    10_u128.pow(12),
    10_u128.pow(13),
    10_u128.pow(14),
];

const MYRIAD_SCALE: [u128; 9] = [
    10_u128.pow(4),
    10_u128.pow(8),
    10_u128.pow(12),
    10_u128.pow(16),
    10_u128.pow(20),
    10_u128.pow(24),
    10_u128.pow(28),
    10_u128.pow(32),
    10_u128.pow(36),
];

const MID_SCALE: [u128; 5] = [
    10_u128.pow(4),
    10_u128.pow(8),
    10_u128.pow(16),
    10_u128.pow(24),
    10_u128.pow(32),
];

const LONG_SCALE: [u128; 4] = [
    10_u128.pow(4),
    10_u128.pow(8),
    10_u128.pow(16),
    10_u128.pow(32),
];

/// Simplified Chinese Informal numerical system.
///
/// By default, this type formats with myriad counting (ten thousands) and
/// ordinary characters.
///
/// This type performs the "informal" rules as defined by the CSS standard. This
/// allows the tens digit to be omitted in some situations.
#[doc = include_str!("./previews/SimplifiedChineseInformal.md")]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct SimplifiedChineseInformal(ChineseOptions);

impl<T> NominalSystem<T> for SimplifiedChineseInformal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let characters = match self.0.usage {
            ChineseUsage::Financial => &SIMPLIFIED_FINANCIAL,
            ChineseUsage::Ordinary => &SIMPLIFIED_ORDINARY,
        };
        format_chinese::<_, false>(characters, &SIMPLIFIED_LARGE, nominal, self.0.scale)
    }
}

/// Simplified Chinese Formal numerical system.
///
/// By default, this type formats with myriad counting (ten thousands) and
/// ordinary characters.
///
/// This type does not implement the "informal" rules as defined by the CSS
/// standard.
#[doc = include_str!("./previews/SimplifiedChineseFormal.md")]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct SimplifiedChineseFormal(ChineseOptions);

impl<T> NominalSystem<T> for SimplifiedChineseFormal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let characters = match self.0.usage {
            ChineseUsage::Financial => &SIMPLIFIED_FINANCIAL,
            ChineseUsage::Ordinary => &SIMPLIFIED_ORDINARY,
        };
        format_chinese::<_, true>(characters, &SIMPLIFIED_LARGE, nominal, self.0.scale)
    }
}

/// Traditional Chinese Formal numerical system.
///
/// By default, this type formats with myriad counting (ten thousands) and
/// ordinary characters.
///
/// This type does not implement the "informal" rules as defined by the CSS
/// standard.
#[doc = include_str!("./previews/TraditionalChineseFormal.md")]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct TraditionalChineseFormal(ChineseOptions);

impl<T> NominalSystem<T> for TraditionalChineseFormal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let characters = match self.0.usage {
            ChineseUsage::Financial => &TRADITIONAL_FINANCIAL,
            ChineseUsage::Ordinary => &TRADITIONAL_ORDINARY,
        };
        format_chinese::<_, true>(characters, &TRADITIONAL_LARGE, nominal, self.0.scale)
    }
}

/// Traditional Chinese Informal numerical system.
///
/// By default, this type formats with myriad counting (ten thousands) and
/// ordinary characters.
///
/// This type performs the "informal" rules as defined by the CSS standard. This
/// allows the tens digit to be omitted in some situations.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct TraditionalChineseInformal(ChineseOptions);

impl<T> NominalSystem<T> for TraditionalChineseInformal
where
    T: Nominal + TryFrom<u128>,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let characters = match self.0.usage {
            ChineseUsage::Financial => &TRADITIONAL_FINANCIAL,
            ChineseUsage::Ordinary => &TRADITIONAL_ORDINARY,
        };
        format_chinese::<_, false>(characters, &TRADITIONAL_LARGE, nominal, self.0.scale)
    }
}

macro_rules! impl_chinese_options {
    ($type:ident) => {
        impl $type {
            /// Returns a variation of this system utilizing financial character
            /// variations instead of ordinary characters.
            ///
            /// For more information on this distinction, see the Wikipedia
            /// section [Ordinary
            /// Numbers](https://en.wikipedia.org/wiki/Chinese_numerals#Ordinary_numerals).
            #[must_use]
            pub const fn financial(mut self) -> Self {
                self.0.usage = ChineseUsage::Financial;
                self
            }

            /// Returns a variation that utilizes the short scale (下數) instead
            /// of the myriad scale (萬進).
            #[must_use]
            pub const fn short_scale(mut self) -> Self {
                self.0.scale = ChineseScale::Short;
                self
            }

            /// Returns a variation that utilizes the mid scale (中數) instead
            /// of the myriad scale (萬進).
            #[must_use]
            pub const fn mid_scale(mut self) -> Self {
                self.0.scale = ChineseScale::Mid;
                self
            }

            /// Returns a variation that utilizes the long scale (上數) instead
            /// of the myriad scale (萬進).
            #[must_use]
            pub const fn long_scale(mut self) -> Self {
                self.0.scale = ChineseScale::Long;
                self
            }
        }
    };
}

impl_chinese_options!(TraditionalChineseFormal);
impl_chinese_options!(TraditionalChineseInformal);
impl_chinese_options!(SimplifiedChineseFormal);
impl_chinese_options!(SimplifiedChineseInformal);

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use chinese_number::NumberToChineseError;

    use super::*;

    #[track_caller]
    fn test_formatting<N>(
        test: &N,
        expected: &[&str],
        chinese_number_fn: impl Fn(u128) -> Result<String, NumberToChineseError>,
    ) where
        N: NominalSystem<u128>,
    {
        let cases = [
            10_u128,
            12,
            100,
            103,
            1000,
            1006,
            1010,
            1011,
            1100,
            1011,
            10_000,
            10_0001,
            10_0000,
            11_0001,
            1_0000_0000,
            1_0000_0001,
            1_0000_0000_0000_0000,
            11111_u128,
            1_0100_0000_0000u128,
            7_4000_0000_0000u128,
            938_4634_6337_0000_0000_0000_0000u128,
            u128::MAX,
        ];
        assert_eq!(expected.len(), cases.len());
        for (index, (case, expected)) in cases.into_iter().zip(expected).enumerate() {
            let result = case.try_to_nominal(test);
            match chinese_number_fn(case) {
                Ok(from_reference_impl) => {
                    assert_eq!(
                        result.expect("unexpected error"),
                        from_reference_impl,
                        "test failure for {case} (idx {index})"
                    );
                    assert_eq!(
                        *expected, from_reference_impl,
                        "reference mismatch (idx {index})"
                    );
                }
                Err(_) => {
                    assert_eq!(result.unwrap_err(), Error::OutOfBounds(case));
                }
            }
        }
    }

    #[test]
    fn simplified_financial() {
        test_formatting(
            &SimplifiedChineseInformal::default().financial(),
            &[
                "拾",
                "拾贰",
                "壹佰",
                "壹佰零叁",
                "壹仟",
                "壹仟零陆",
                "壹仟零壹拾",
                "壹仟零壹拾壹",
                "壹仟壹佰",
                "壹仟零壹拾壹",
                "壹万",
                "拾万零壹",
                "拾万",
                "拾壹万零壹",
                "壹亿",
                "壹亿零壹",
                "壹京",
                "壹万壹仟壹佰壹拾壹",
                "壹兆零壹佰亿",
                "柒兆肆仟亿",
                "玖佰叁拾捌秭肆仟陆佰叁拾肆垓陆仟叁佰叁拾柒京",
                "叁佰肆拾涧贰仟捌佰贰拾叁沟陆仟陆佰玖拾贰穰零玖佰叁拾捌秭肆仟陆佰叁拾肆垓陆仟叁佰叁拾柒京肆仟陆佰零柒兆肆仟叁佰壹拾柒亿陆仟捌佰贰拾壹万壹仟肆佰伍拾伍",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_ten_thousand(
                    chinese_number::ChineseVariant::Simple,
                    chinese_number::ChineseCase::Upper,
                    n,
                ))
            },
        );
    }

    #[test]
    fn simplified_financial_low() {
        test_formatting(
            &SimplifiedChineseInformal::default()
                .financial()
                .short_scale(),
            &[
                "拾",
                "拾贰",
                "壹佰",
                "壹佰零叁",
                "壹仟",
                "壹仟零陆",
                "壹仟零壹拾",
                "壹仟零壹拾壹",
                "壹仟壹佰",
                "壹仟零壹拾壹",
                "壹万",
                "壹亿零壹",
                "壹亿",
                "壹亿壹万零壹",
                "壹垓",
                "壹垓零壹",
                "壹万壹仟壹佰壹拾壹",
                "壹万壹仟壹佰壹拾壹",
                "壹涧零壹穰",
                "柒涧肆沟",
                "",
                "",
            ],
            |n| {
                chinese_number::from_u128_to_chinese_low(
                    chinese_number::ChineseVariant::Simple,
                    chinese_number::ChineseCase::Upper,
                    n,
                )
            },
        );
    }

    #[test]
    fn simplified_financial_high() {
        test_formatting(
            &SimplifiedChineseInformal::default()
                .financial()
                .long_scale(),
            &[
                "拾",
                "拾贰",
                "壹佰",
                "壹佰零叁",
                "壹仟",
                "壹仟零陆",
                "壹仟零壹拾",
                "壹仟零壹拾壹",
                "壹仟壹佰",
                "壹仟零壹拾壹",
                "壹万",
                "拾万零壹",
                "拾万",
                "拾壹万零壹",
                "壹亿",
                "壹亿零壹",
                "壹兆",
                "壹万壹仟壹佰壹拾壹",
                "壹万零壹佰亿",
                "柒万肆仟亿",
                "玖佰叁拾捌亿肆仟陆佰叁拾肆万陆仟叁佰叁拾柒兆",
                "叁佰肆拾万贰仟捌佰贰拾叁京陆仟陆佰玖拾贰万零玖佰叁拾捌亿肆仟陆佰叁拾肆万陆仟叁佰叁拾柒兆肆仟陆佰零柒万肆仟叁佰壹拾柒亿陆仟捌佰贰拾壹万壹仟肆佰伍拾伍",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_high(
                    chinese_number::ChineseVariant::Simple,
                    chinese_number::ChineseCase::Upper,
                    n,
                ))
            },
        );
    }

    #[test]
    fn simplified_financial_mid() {
        test_formatting(
            &SimplifiedChineseInformal::default().financial().mid_scale(),
            &[
                "拾",
                "拾贰",
                "壹佰",
                "壹佰零叁",
                "壹仟",
                "壹仟零陆",
                "壹仟零壹拾",
                "壹仟零壹拾壹",
                "壹仟壹佰",
                "壹仟零壹拾壹",
                "壹万",
                "拾万零壹",
                "拾万",
                "拾壹万零壹",
                "壹亿",
                "壹亿零壹",
                "壹兆",
                "壹万壹仟壹佰壹拾壹",
                "壹万零壹佰亿",
                "柒万肆仟亿",
                "玖佰叁拾捌京肆仟陆佰叁拾肆万陆仟叁佰叁拾柒兆",
                "叁佰肆拾万贰仟捌佰贰拾叁垓陆仟陆佰玖拾贰万零玖佰叁拾捌京肆仟陆佰叁拾肆万陆仟叁佰叁拾柒兆肆仟陆佰零柒万肆仟叁佰壹拾柒亿陆仟捌佰贰拾壹万壹仟肆佰伍拾伍",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_middle(
                    chinese_number::ChineseVariant::Simple,
                    chinese_number::ChineseCase::Upper,
                    n,
                ))
            },
        );
    }
    #[test]
    fn simplified() {
        test_formatting(
            &SimplifiedChineseInformal::default(),
            &[
                "十",
                "十二",
                "一百",
                "一百零三",
                "一千",
                "一千零六",
                "一千零一十",
                "一千零一十一",
                "一千一百",
                "一千零一十一",
                "一万",
                "十万零一",
                "十万",
                "十一万零一",
                "一亿",
                "一亿零一",
                "一京",
                "一万一千一百一十一",
                "一兆零一百亿",
                "七兆四千亿",
                "九百三十八秭四千六百三十四垓六千三百三十七京",
                "三百四十涧二千八百二十三沟六千六百九十二穰零九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七亿六千八百二十一万一千四百五十五",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_ten_thousand(
                    chinese_number::ChineseVariant::Simple,
                    chinese_number::ChineseCase::Lower,
                    n,
                ))
            },
        );
    }

    #[test]
    fn traditional() {
        test_formatting(
            &TraditionalChineseInformal::default(),
            &[
                "十",
                "十二",
                "一百",
                "一百零三",
                "一千",
                "一千零六",
                "一千零一十",
                "一千零一十一",
                "一千一百",
                "一千零一十一",
                "一萬",
                "十萬零一",
                "十萬",
                "十一萬零一",
                "一億",
                "一億零一",
                "一京",
                "一萬一千一百一十一",
                "一兆零一百億",
                "七兆四千億",
                "九百三十八秭四千六百三十四垓六千三百三十七京",
                "三百四十澗二千八百二十三溝六千六百九十二穰零九百三十八秭四千六百三十四垓六千三百三十七京四千六百零七兆四千三百一十七億六千八百二十一萬一千四百五十五",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_ten_thousand(
                    chinese_number::ChineseVariant::Traditional,
                    chinese_number::ChineseCase::Lower,
                    n,
                ))
            },
        );
    }

    #[test]
    fn traditional_financial() {
        test_formatting(
            &TraditionalChineseInformal::default().financial(),
            &[
                "拾",
                "拾貳",
                "壹佰",
                "壹佰零參",
                "壹仟",
                "壹仟零陸",
                "壹仟零壹拾",
                "壹仟零壹拾壹",
                "壹仟壹佰",
                "壹仟零壹拾壹",
                "壹萬",
                "拾萬零壹",
                "拾萬",
                "拾壹萬零壹",
                "壹億",
                "壹億零壹",
                "壹京",
                "壹萬壹仟壹佰壹拾壹",
                "壹兆零壹佰億",
                "柒兆肆仟億",
                "玖佰參拾捌秭肆仟陸佰參拾肆垓陸仟參佰參拾柒京",
                "參佰肆拾澗貳仟捌佰貳拾參溝陸仟陸佰玖拾貳穰零玖佰參拾捌秭肆仟陸佰參拾肆垓陸仟參佰參拾柒京肆仟陸佰零柒兆肆仟參佰壹拾柒億陸仟捌佰貳拾壹萬壹仟肆佰伍拾伍",
            ],
            |n| {
                Ok(chinese_number::from_u128_to_chinese_ten_thousand(
                    chinese_number::ChineseVariant::Traditional,
                    chinese_number::ChineseCase::Upper,
                    n,
                ))
            },
        );
    }
}
