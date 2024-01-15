use chinese_number::{
    ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese, NumberToChineseError,
};

use crate::{NominalString, NominalSystem};

/// A Chinese nominal system.
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
    T: NumberToChinese + Copy,
{
    type Error = NumberToChineseError;

    fn try_format_nominal(&self, numeric: T) -> Result<NominalString, Self::Error> {
        let formatted = numeric.to_chinese(self.variant, self.case, self.method)?;

        Ok(NominalString::from(formatted))
    }
}
