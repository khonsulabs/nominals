# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->
## Unreleased

### Breaking Changes

- `HangeulInformal` has been renamed to `HanjaInformal`, its correct name.
- `DigitCollection::len` and `DigitCollection::digit` no longer receive a
  `digit_index` parameter.
- `DigitCollection::Fallback` is a new associated type that allows a digit
  collection to define a fallback digit set. `NoFallback` is a new type that can
  be used when no fallback should be used.

### Added

- `DigitCollection::fixed` is a new function that indicates the collection
  should only ever be used to produce a single digit.

### Fixed

- `CjkHeavenlyStem` and `CjkEarthlyBranch` now fall back to `CjkDecimal` in a
  fashion that matches the cSS specification. Specifically once the symbols have
  been exhausted, the entire nominal formatting is produced by the fallback
  rather than only formatting the remaining digits with the fallback.

## v0.2.2 (2023-01-18)

### Changed

- `Urdu` and `Khmer` now utilize re-exports rather than type aliases. This
  allows the type constructors to be invoked.

### Fixed

- Ethiopic now returns the correct result for 1.

## v0.2.1 (2023-01-18)

### Fixed

- A compilation error for no-alloc targets has been fixed.

## v0.2.0 (2023-01-18)

### Breaking Changes

- `RomanLowercase` has been renamed to `RomanLower` for consistency.
- `HiraganaAiueo` and `KatakanaAiueo` have had `Aiueo` removed from their names
  to match the CSS naming and reflect modern expectations.
- `Urdu` has been removed. The `Persian` Unicode range is reused for `Urdu`
  numerals, and the different symbols are a result of using an Urdu or Persian
  font.
- `DigitCollection::len` and `DigitCollection::digit` now take a `digit_index`
  parameter, allowing a digit collection to delegate specific digits to other
  implementations.

### Changed

- `NominalString::INLINE_CAPACITY` is now 62, and the type has been optimized to
  avoid zero-initializing data unnecessarily.

### Added

- `NominalString` now implements `Clone`.
- `NominalString::new_reverse()` returns an empty string that is optimized for
  `push_front` operations.
- `AdditiveSet` is a new `NominalSystem` implementation that implements additive
  nominal systems. The existing Roman numeral implementation is now utilizing
  this type.
- These nominal systems have been added from the CSS counter-styles spec:

  - `ArmenianLower`
  - `ArmenianUpper`
  - `Bengali`
  - `Cambodian`
  - `CjkDecimal`
  - `CjkEarthlyBranch`
  - `CjkHeavenlyStem`
  - `Devanagari`
  - `Ethiopic`
  - `Georgian`
  - `Gujarati`
  - `Gurmukhi`
  - `HangeulFormal`
  - `HangeulInformal`
  - `HanjaFormal`
  - `JapaneseFormal`
  - `JapaneseInformal`
  - `Kannada`
  - `Lao`
  - `Malayalam`
  - `Mongolian`
  - `Myanmar`
  - `Oriya`
  - `Tamil`
  - `Telugu`
  - `Thai`
  - `Tibetan`

## v0.1.0 (2023-01-15)

This is the initial release.
