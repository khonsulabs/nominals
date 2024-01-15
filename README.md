# numbering

This is a `no_std` compatible crate that provides number formatting for list
representations in multiple numbering systems. It was written to power the
`List` widget in [Cushy][cushy]. This crate takes some inspiration from
[Typst][typst]'s implementation of list numbering, but fresh implementations
were created to create this crate's API.

- [x] Basic formatting
- [ ] Separators
- [ ] "Any" type

## Systems Supported

- Arabic (0-9)
- Letter Upper (A-Z)
- Letter Lower (a-z)
- Hebrew
- Simplified Chinese
- Traditional Chinese
- Hiragana Aiueo
- Hiragana Iroha
- Katakana Aiueo
- Katakana Iroha
- Korean Jamo
- Korean Syllable

## Systems Desired

Any and all. Especially any listed in the `list-style-type` specification in
CSS. Pull requests welcome and accepted!

[cushy]: https://github.com/khonsulabs/cushy
[typst]: https://github.com/typst/typst
