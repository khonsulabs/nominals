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

### Changed

- `NominalString::INLINE_CAPACITY` is now 62, and the type has been optimized to
  avoid zero-initializing data unnecessarily.

### Added

- `NominalString` now implements `Clone`.

## v0.1.0 (2023-12-18)

This is the initial alpha release of Cushy.
