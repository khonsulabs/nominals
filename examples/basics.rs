//! Shows basic interactions with this library.

// begin rustme snippet: readme
use nominals::{Decimal, DigitCollection, Error, LetterLower, Nominal};

fn main() {
    // Formatting a nominal is as easy as calling to_nominal with the desired
    // nominal system.
    assert_eq!(0_u32.to_nominal(&Decimal), "0");
    // By default, alphabet systems treat their first symbol as 0 in the ones
    // place, but treat the first symbol as 1 in every other digit place.
    assert_eq!(0_u32.to_nominal(&LetterLower), "a");
    assert_eq!(1_u32.to_nominal(&LetterLower), "b");
    assert_eq!(26_u32.to_nominal(&LetterLower), "aa");

    // This behavior can be changed by using the one_based helper:
    assert_eq!(1_u32.to_nominal(&LetterLower.one_based()), "a");
    assert_eq!(2_u32.to_nominal(&LetterLower.one_based()), "b");
    assert_eq!(27_u32.to_nominal(&LetterLower.one_based()), "aa");
    // When a nominal can't be formatted with `to_nominal`, it falls back to
    // Decimal.
    assert_eq!(0_u32.to_nominal(&LetterLower.one_based()), "0");
    assert_eq!(
        0_u32.try_to_nominal(&LetterLower.one_based()).unwrap_err(),
        Error::NoZeroSymbol
    );
}
// end rustme snippet: readme

#[test]
fn runs() {
    main()
}
