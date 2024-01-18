use crate::{Error, Nominal, NominalString, NominalSystem, OutOfMemoryError};

/// Ethiopic numerical system.
#[doc = include_str!("./previews/Ethiopic.md")]
pub struct Ethiopic;

const ONES: [char; 10] = [
    '\0', '\u{1369}', '\u{136A}', '\u{136B}', '\u{136C}', '\u{136D}', '\u{136E}', '\u{136F}',
    '\u{1370}', '\u{1371}',
];
const TENS: [char; 10] = [
    '\0', '\u{1372}', '\u{1373}', '\u{1374}', '\u{1375}', '\u{1376}', '\u{1377}', '\u{1378}',
    '\u{1379}', '\u{137A}',
];

fn format_ethiopic<T: Nominal>(nominal: T) -> Result<NominalString, OutOfMemoryError> {
    let ten = T::from(10);

    let mut formatted = NominalString::new_reverse();

    let mut remaining = nominal;
    for group_index in 0.. {
        if remaining.is_zero() {
            break;
        }

        let first = remaining % ten;
        remaining /= ten;
        let second = remaining % ten;
        remaining /= ten;

        let first_is_zero = first.is_zero();
        let second_is_zero = second.is_zero();
        let first_is_one = first == T::from(1);
        let group_is_odd = group_index % 2 == 1;
        let not_first_group = group_index > 0;

        if not_first_group {
            if !group_is_odd {
                formatted.try_push_front('\u{137C}')?;
            } else if !(first_is_zero && second_is_zero) {
                formatted.try_push_front('\u{137B}')?;
            }
        }

        let remove_digits = (first_is_zero && second_is_zero)
            || ((remaining.is_zero() || group_is_odd)
                && not_first_group
                && second_is_zero
                && first_is_one);

        if !remove_digits {
            if !first_is_zero {
                formatted.try_push_front(ONES[first.as_usize()])?;
            }

            if !second_is_zero {
                formatted.try_push_front(TENS[second.as_usize()])?;
            }
        }
    }

    Ok(formatted)
}

impl<T> NominalSystem<T> for Ethiopic
where
    T: Nominal,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        if nominal.is_zero() {
            return Err(Error::NoZeroSymbol);
        }
        format_ethiopic(nominal).map_err(|_| Error::OutOfMemory(nominal))
    }
}

#[test]
fn ethiopic() {
    assert_eq!(1_u32.to_nominal(&Ethiopic), "፩");
    assert_eq!(100_u32.to_nominal(&Ethiopic), "፻");
    assert_eq!(101_u32.to_nominal(&Ethiopic), "፻፩");
    assert_eq!(78_010_092_u32.to_nominal(&Ethiopic), "፸፰፻፩፼፺፪");
    assert_eq!(780_100_000_092_u64.to_nominal(&Ethiopic), "፸፰፻፩፼፼፺፪");
}
