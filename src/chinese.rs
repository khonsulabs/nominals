use chinese_number::{ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese};

use crate::{Error, Nominal, NominalString, NominalSystem};

/// A Chinese nominal system.
#[doc = include_str!("./previews/TraditionalChinese.md")]
#[doc = include_str!("./previews/SimplifiedChinese.md")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Chinese {
    /// Determines whether to format using simplified or traditional Chinese.
    pub variant: ChineseVariant,
    /// Determines the counting method. The default counting method is
    /// [`ChineseCountMethod::TenThousand`].
    pub method: ChineseCountMethod,
    /// Determines whether uppercase or lowercase characters are used. The
    /// default case is [`ChineseCase::Upper`].
    pub case: ChineseCase,
}

impl Chinese {
    /// Returns a traditional Chinese nominal system.
    #[must_use]
    pub const fn traditional() -> Self {
        Self {
            variant: ChineseVariant::Traditional,
            method: ChineseCountMethod::TenThousand,
            case: ChineseCase::Upper,
        }
    }

    /// Returns a simplified Chinese nominal system.
    #[must_use]
    pub const fn simplified() -> Self {
        Self {
            variant: ChineseVariant::Simple,
            method: ChineseCountMethod::TenThousand,
            case: ChineseCase::Upper,
        }
    }

    /// Returns this system as a lowercase system.
    #[must_use]
    pub const fn lowercase(mut self) -> Self {
        self.case = ChineseCase::Lower;
        self
    }

    /// Returns this system with the updated counting `method`.
    #[must_use]
    pub const fn method(mut self, method: ChineseCountMethod) -> Self {
        self.method = method;
        self
    }
}

impl<T> NominalSystem<T> for Chinese
where
    T: Nominal + NumberToChinese + Copy,
    <T as TryFrom<usize>>::Error: core::fmt::Debug,
    <T as TryInto<usize>>::Error: core::fmt::Debug,
{
    fn try_format_nominal(&self, numeric: T) -> Result<NominalString, Error<T>> {
        let formatted = numeric
            .to_chinese(self.variant, self.case, self.method)
            .map_err(|_| Error::OutOfBounds(numeric))?;

        Ok(NominalString::from(formatted))
    }
}
