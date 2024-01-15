use chinese_number::{
    ChineseCase, ChineseCountMethod, ChineseVariant, NumberToChinese, NumberToChineseError,
};
use ecow::EcoString;

use crate::Numbering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Chinese {
    pub variant: ChineseVariant,
    pub method: ChineseCountMethod,
    pub case: ChineseCase,
}

impl Chinese {
    pub const fn traditional() -> Self {
        Self {
            variant: ChineseVariant::Traditional,
            method: ChineseCountMethod::TenThousand,
            case: ChineseCase::Upper,
        }
    }

    pub const fn simplified() -> Self {
        Self {
            variant: ChineseVariant::Simple,
            method: ChineseCountMethod::TenThousand,
            case: ChineseCase::Upper,
        }
    }

    pub const fn lowercase(mut self) -> Self {
        self.case = ChineseCase::Lower;
        self
    }

    pub const fn method(mut self, method: ChineseCountMethod) -> Self {
        self.method = method;
        self
    }
}

impl<T> Numbering<T> for Chinese
where
    T: NumberToChinese + Copy,
{
    type Error = NumberToChineseError;

    fn try_format_number(&self, numeric: T) -> Result<EcoString, Self::Error> {
        let formatted = numeric.to_chinese(self.variant, self.case, self.method)?;

        Ok(EcoString::from(formatted))
    }
}
