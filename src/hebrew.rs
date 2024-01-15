use crate::{Error, NominalString, NominalSystem, UnsignedInteger};

/// Hebrew numerals.
pub struct Hebrew;

impl<T> NominalSystem<T> for Hebrew
where
    T: UnsignedInteger + TryFrom<u32> + From<u8>,
{
    type Error = Error;

    fn try_format_nominal(&self, mut remaining: T) -> Result<NominalString, Self::Error> {
        if remaining.is_zero() {
            return Err(Error::NoZeroSymbol);
        }

        let fifteen = T::from(15);
        let sixteen = T::from(16);

        let mut formatted = NominalString::default();
        'symbol_loop: for (symbol, value) in [
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
                    formatted.try_push_str("ט״ו")?;
                    break 'symbol_loop;
                } else if remaining == sixteen {
                    formatted.try_push_str("ט״ז")?;
                    break 'symbol_loop;
                }

                // When a single symbol is used to represent a number, the
                // symbol is wrapped in geresh and gershayim characters to
                // distinguish it from a word.
                let single_symbol = value == remaining && formatted.is_empty();
                if single_symbol {
                    formatted.try_push('׳')?;
                }
                remaining = remaining - value;
                formatted.try_push(symbol)?;
                if single_symbol {
                    formatted.try_push('״')?;
                    break;
                }
            }
        }

        Ok(formatted)
    }
}

#[test]
fn hebrew() {
    assert_eq!(Hebrew.format_nominal(997_u32), "תתקצז");
    assert_eq!(Hebrew.format_nominal(1_u32), "׳א״");
    assert_eq!(Hebrew.format_nominal(15_u32), "ט״ו");
    assert_eq!(Hebrew.format_nominal(16_u32), "ט״ז");
    assert_eq!(Hebrew.try_format_nominal(0_u32), Err(Error::NoZeroSymbol));
}
