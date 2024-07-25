use crate::{Error, Nominal, NominalString, NominalSystem};

/// A set of enumerated symbols that form a [`NominalSystem`].
pub struct EnumeratedSet<const N: usize, const HAS_ZERO: bool> {
    symbols: [&'static str; N],
}

impl<const N: usize> EnumeratedSet<N, true> {
    /// Creates a new enumerated set containing `symbols`,
    /// representing numerals in ascending order.
    #[must_use]
    pub const fn new(symbols: [&'static str; N]) -> Self {
        Self { symbols }
    }
}

impl<const N: usize> EnumeratedSet<N, false> {
    /// Creates a new enumerated set containing `symbols`,
    /// representing numerals in ascending order.
    #[must_use]
    pub const fn zeroless(symbols: [&'static str; N]) -> Self {
        Self { symbols }
    }
}

impl<const N: usize, const HAS_ZERO: bool, T> NominalSystem<T> for EnumeratedSet<N, HAS_ZERO>
where
    T: Nominal,
{
    fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
        let n: usize = nominal
            .try_into()
            .map_err(|_| Error::OutOfBounds(nominal))?;

        if !HAS_ZERO && nominal.is_zero() {
            Err(Error::NoZeroSymbol)
        } else if let Some(&symbol) = self.symbols.get(n - usize::from(!HAS_ZERO)) {
            Ok(symbol.into())
        } else {
            Err(Error::OutOfBounds(nominal))
        }
    }
}

macro_rules! impl_enum_set {
    ($(#$doc:tt)* $name:ident, $kind:ident, $symbols:expr) => {
        $(#$doc)*
        // When adding a new variant and getting an error here, either
        // temporarily comment this out or add an empty file until the new
        // variant has been added to the previews example.
        #[doc = include_str!(concat!("./previews/",stringify!($name), ".md"))]
        #[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub struct $name;

        impl<T> NominalSystem<T> for $name
        where
            T: Nominal,
        {
            fn try_format_nominal(&self, nominal: T) -> Result<NominalString, Error<T>> {
                EnumeratedSet::$kind($symbols)
                .try_format_nominal(nominal)
            }
        }
    };
}

impl_enum_set! {
    /// Circled numbers, ranging from 0 to 50.
    CircledNumber, new, [ 
        "⓪", "①", "②", "③", "④", "⑤", "⑥", "⑦", "⑧", "⑨", "⑩",
        "⑪", "⑫", "⑬", "⑭", "⑮", "⑯", "⑰", "⑱", "⑲", "⑳",
        "㉑", "㉒", "㉓", "㉔", "㉕", "㉖", "㉗", "㉘", "㉙", "㉚",
        "㉛", "㉜", "㉝", "㉞", "㉟", "㊱", "㊲", "㊳", "㊴", "㊵",
        "㊶", "㊷", "㊸", "㊹", "㊺", "㊻", "㊼", "㊽", "㊾", "㊿"
    ]
}

impl_enum_set! {
    /// Doubly-circled numbers, ranging from 1 to 10.
    DoubleCircledNumber, zeroless, ["⓵", "⓶", "⓷", "⓸", "⓹", "⓺", "⓻", "⓼", "⓽", "⓾"]
}
