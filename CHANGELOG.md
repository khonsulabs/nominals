# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- markdownlint-disable no-duplicate-heading -->

## Unreleased

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

## v0.1.0 (2023-12-18)

This is the initial alpha release of Cushy.
