# nominals

A crate for formatting nominal indicators in various systems.

This type of formatting can be used for list identifiers when creating ordered
lists similar to HTML's `<ol>` tag. The crate was written to power the
`List` widget in [Cushy][cushy].

This crate's original implementation was inspired by [Typst][typst]'s
implementation of list numbering. However, this crate is a fresh implementation
with newly-written implementations and types.

## Systems Supported

| System       | Supported |
|--------------|-----------|
| Arabic (0-9)             | ✅ |
| Chinese (Simplified)     | ✅ |
| Chinese (Traditional)    | ✅ |
| Eastern Arabic           | ✅ |
| Greek Lowercase          | ✅ |
| Greek Uppercase          | ✅ |
| Hebrew                   | ✅ |
| Hiragana Aiueo           | ✅ |
| Hiragana Iroha           | ✅ |
| Katakana Aiueo           | ✅ |
| Katakana Iroha           | ✅ |
| Korean Jamo              | ✅ |
| Korean Syllable          | ✅ |
| Letter Lowercase (a-z)   | ✅ |
| Letter Upper (A-Z)       | ✅ |
| Persian                  | ✅ |
| Roman Lowercase          | ✅ |
| Roman Uppercase          | ✅ |
| Urdu                     | ✅ |
| `armenian` | |
| `bengali` | |
| `cambodian` | |
| `cjk-earthly-branch` | |
| `cjk-heavenly-stem` | |
| `devangari` | |
| `ethiopic-numeric` | |
| `georgian` | |
| `gujarati` | |
| `gurmukhi` | |
| `japanese-formal` | |
| `japanese-informal` | |
| `kannada` | |
| `lao` | |
| `malayalam` | |
| `mongolian` | |
| `myanmar` | |
| `oriya` | |
| `tamil` | |
| `telegu` | |
| `thai` | |
| `tibetan` | |
| `kannada` | |

All missing systems are listed using their [CSS
list-style-type][list-style-type] identifier. Pull requests are welcome for any
and all systems!

## `no_std` support

This crate is `no_std` compatible, and can operate both with and without
`alloc`. Some systems, such as Chinese, require `alloc` due to reliance on
dependencies that require alloc.

[cushy]: https://github.com/khonsulabs/cushy
[typst]: https://github.com/typst/typst
[list-style-type]: https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type
