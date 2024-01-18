# Nominals

<!-- This file is generated by `rustme`. Ensure you're editing the source in the .rustme/ directory --!>
<!-- markdownlint-disable first-line-h1 -->

![nominals is considered alpha](https://img.shields.io/badge/status-alpha-orange)
[![crate version](https://img.shields.io/crates/v/nominals.svg)](https://crates.io/crates/nominals)
[![Documentation for `v0.2.1`](https://img.shields.io/badge/docs-v0.2.1-informational)](https://docs.rs/nominals)

This type of formatting can be used for list identifiers when creating ordered
lists similar to HTML's `<ol>` tag. The crate was written to power the
`List` widget in [Cushy][cushy].

This crate's original implementation was inspired by [Typst][typst]'s
implementation of list numbering. However, this crate is a fresh implementation
with newly-written implementations and types.

```rust
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
```

## `no_std` support

This crate is `no_std` compatible, and can operate both with and without
`alloc`.

[cushy]: https://github.com/khonsulabs/cushy
[typst]: https://github.com/typst/typst

## Supported Systems

- [`Decimal`](https://docs.rs/nominals/latest/nominals/struct.Decimal.html): 0‎ 1‎ 2‎ 3‎ 4‎ …‎ 9‎ 10‎ 11‎ 12‎ …‎ 99‎ 100‎ 101‎ 102
- [`LetterLower`](https://docs.rs/nominals/latest/nominals/struct.LetterLower.html): a‎ b‎ c‎ d‎ e‎ …‎ z‎ aa‎ ab‎ ac‎ …‎ yz‎ za‎ zb‎ zc
- [`LetterUpper`](https://docs.rs/nominals/latest/nominals/struct.LetterUpper.html): A‎ B‎ C‎ D‎ E‎ …‎ Z‎ AA‎ AB‎ AC‎ …‎ YZ‎ ZA‎ ZB‎ ZC
- [`RomanLower`](https://docs.rs/nominals/latest/nominals/struct.RomanLower.html): i‎ ii‎ iii‎ iv‎ v‎ vi‎ …‎ ix‎ x‎ xi‎ …‎ mmmcmxcix‎ i̅v̅‎ i̅v̅i‎ …‎ d̅m̅l̅c̅i̅x̅cmxcix‎ m̅‎ m̅i
- [`RomanUpper`](https://docs.rs/nominals/latest/nominals/struct.RomanUpper.html): I‎ II‎ III‎ IV‎ V‎ VI‎ …‎ IX‎ X‎ XI‎ …‎ MMMCMXCIX‎ I̅V̅‎ I̅V̅I‎ …‎ D̅M̅L̅C̅I̅X̅CMXCIX‎ M̅‎ M̅I
- [`ArmenianLower`](https://docs.rs/nominals/latest/nominals/struct.ArmenianLower.html): ա‎ բ‎ գ‎ …‎ թ‎ ժ‎ ժա‎ ժբ‎ …‎ ղթ‎ ճ‎ ճա
- [`ArmenianUpper`](https://docs.rs/nominals/latest/nominals/struct.ArmenianUpper.html): Ա ‎ Բ‎ Գ‎ …‎ Թ‎ Ժ‎ ԺԱ ‎ ԺԲ‎ …‎ ՂԹ‎ Ճ‎ ՃԱ 
- [`Bengali`](https://docs.rs/nominals/latest/nominals/struct.Bengali.html): ০‎ ১‎ ২‎ ৩‎ ৪‎ …‎ ৯‎ ১০‎ ১১‎ ১২‎ …‎ ৯৯‎ ১০০‎ ১০১‎ ১০২
- [`Cambodian`](https://docs.rs/nominals/latest/nominals/struct.Cambodian.html): ០‎ ១‎ ២‎ ៣‎ ៤‎ …‎ ៩‎ ១០‎ ១១‎ ១២‎ …‎ ៩៩‎ ១០០‎ ១០១‎ ១០២
- [`SimplifiedChineseInformal`](https://docs.rs/nominals/latest/nominals/struct.SimplifiedChineseInformal.html): 零‎ 一‎ 二‎ …‎ 九‎ 十‎ 十一‎ …‎ 十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 一百‎ 一百零一
- [`TraditionalChineseInformal`](https://docs.rs/nominals/latest/nominals/struct.TraditionalChineseInformal.html): 零‎ 一‎ 二‎ …‎ 九‎ 十‎ 十一‎ …‎ 十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 一百‎ 一百零一
- [`SimplifiedChineseFormal`](https://docs.rs/nominals/latest/nominals/struct.SimplifiedChineseFormal.html): 零‎ 一‎ 二‎ …‎ 九‎ 一十‎ 一十一‎ …‎ 一十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 一百‎ 一百零一
- [`TraditionalChineseFormal`](https://docs.rs/nominals/latest/nominals/struct.TraditionalChineseFormal.html): 零‎ 一‎ 二‎ …‎ 九‎ 一十‎ 一十一‎ …‎ 一十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 一百‎ 一百零一
- [`CjkDecimal`](https://docs.rs/nominals/latest/nominals/struct.CjkDecimal.html): 〇‎ 一‎ 二‎ 三‎ 四‎ …‎ 九‎ 一〇‎ 一一‎ 一二‎ …‎ 九九‎ 一〇〇‎ 一〇一‎ 一〇二
- [`CjkEarthlyBranch`](https://docs.rs/nominals/latest/nominals/struct.CjkEarthlyBranch.html): 子‎ 丑‎ 寅‎ 卯‎ 辰‎ …‎ 亥‎ 一子‎ 一丑‎ 一寅‎ …‎ 九亥‎ 一〇子‎ 一〇丑‎ 一〇寅
- [`CjkHeavenlyStem`](https://docs.rs/nominals/latest/nominals/struct.CjkHeavenlyStem.html): 甲‎ 乙‎ 丙‎ 丁‎ 戊‎ …‎ 癸‎ 一甲‎ 一乙‎ 一丙‎ …‎ 九癸‎ 一〇甲‎ 一〇乙‎ 一〇丙
- [`Devanagari`](https://docs.rs/nominals/latest/nominals/struct.Devanagari.html): ०‎ १‎ २‎ ३‎ ४‎ …‎ ९‎ १०‎ ११‎ १२‎ …‎ ९९‎ १००‎ १०१‎ १०२
- [`EasternArabic`](https://docs.rs/nominals/latest/nominals/struct.EasternArabic.html): ٠‎ ١‎ ٢‎ ٣‎ ٤‎ …‎ ٩‎ ١٠‎ ١١‎ ١٢‎ …‎ ٩٩‎ ١٠٠‎ ١٠١‎ ١٠٢
- [`Ethiopic`](https://docs.rs/nominals/latest/nominals/struct.Ethiopic.html): ፪‎ ፫‎ …‎ ፲‎ ፲፩‎ ፲፪‎ …‎ ፺፱‎ ፻‎ ፻፩‎ …‎ ፱፻፺፱‎ ፲፻‎ ፲፻፩
- [`Georgian`](https://docs.rs/nominals/latest/nominals/struct.Georgian.html): ა‎ ბ‎ გ‎ …‎ თ‎ ი‎ ია‎ იბ‎ …‎ ჟთ‎ რ‎ რა
- [`GreekLower`](https://docs.rs/nominals/latest/nominals/struct.GreekLower.html): α‎ β‎ γ‎ δ‎ ε‎ …‎ ω‎ αα‎ αβ‎ αγ‎ …‎ ψω‎ ωα‎ ωβ‎ ωγ
- [`GreekUpper`](https://docs.rs/nominals/latest/nominals/struct.GreekUpper.html): Α‎ Β‎ Γ‎ Δ‎ Ε‎ …‎ Ω‎ ΑΑ‎ ΑΒ‎ ΑΓ‎ …‎ ΨΩ‎ ΩΑ‎ ΩΒ‎ ΩΓ
- [`Gujarati`](https://docs.rs/nominals/latest/nominals/struct.Gujarati.html): ૦‎ ૧‎ ૨‎ ૩‎ ૪‎ …‎ ૯‎ ૧૦‎ ૧૧‎ ૧૨‎ …‎ ૯૯‎ ૧૦૦‎ ૧૦૧‎ ૧૦૨
- [`Gurmukhi`](https://docs.rs/nominals/latest/nominals/struct.Gurmukhi.html): ੦‎ ੧‎ ੨‎ ੩‎ ੪‎ …‎ ੯‎ ੧੦‎ ੧੧‎ ੧੨‎ …‎ ੯੯‎ ੧੦੦‎ ੧੦੧‎ ੧੦੨
- [`HangeulFormal`](https://docs.rs/nominals/latest/nominals/struct.HangeulFormal.html): 영‎ 일‎ 이‎ …‎ 구‎ 일십‎ 일십일‎ …‎ 일십구‎ 이십‎ 이십일‎ …‎ 구십구‎ 일백‎ 일백일
- [`HangeulInformal`](https://docs.rs/nominals/latest/nominals/struct.HangeulInformal.html): 零‎ 一‎ 二‎ …‎ 九‎ 十‎ 十一‎ …‎ 十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 百‎ 百一
- [`HangeulJamo`](https://docs.rs/nominals/latest/nominals/struct.HangeulJamo.html): ㄱ‎ ㄴ‎ ㄷ‎ ㄹ‎ ㅁ‎ …‎ ㅎ‎ ㄱㄱ‎ ㄱㄴ‎ ㄱㄷ‎ …‎ ㅍㅎ‎ ㅎㄱ‎ ㅎㄴ‎ ㅎㄷ
- [`HangeulSyllable`](https://docs.rs/nominals/latest/nominals/struct.HangeulSyllable.html): 가‎ 나‎ 다‎ 라‎ 마‎ …‎ 하‎ 가가‎ 가나‎ 가다‎ …‎ 파하‎ 하가‎ 하나‎ 하다
- [`HanjaFormal`](https://docs.rs/nominals/latest/nominals/struct.HanjaFormal.html): 零‎ 壹‎ 貳‎ …‎ 九‎ 壹拾‎ 壹拾壹‎ …‎ 壹拾九‎ 貳拾‎ 貳拾壹‎ …‎ 九拾九‎ 壹百‎ 壹百壹
- [`Hebrew`](https://docs.rs/nominals/latest/nominals/struct.Hebrew.html): ׳א״‎ ׳ב״‎ ׳ג״‎ …‎ ׳ט״‎ ׳י״‎ …‎ יג‎ יד‎ ט״ו‎ ט״ז‎ יז‎ …‎ שצו‎ שצז‎ שצח‎ שצט‎ ׳ת״
- [`HexLower`](https://docs.rs/nominals/latest/nominals/struct.HexLower.html): 0‎ 1‎ 2‎ 3‎ 4‎ …‎ f‎ 10‎ 11‎ 12‎ …‎ ff‎ 100‎ 101‎ 102
- [`HexUpper`](https://docs.rs/nominals/latest/nominals/struct.HexUpper.html): 0‎ 1‎ 2‎ 3‎ 4‎ …‎ F‎ 10‎ 11‎ 12‎ …‎ FF‎ 100‎ 101‎ 102
- [`Hiragana`](https://docs.rs/nominals/latest/nominals/struct.Hiragana.html): あ‎ い‎ う‎ え‎ お‎ …‎ ん‎ ああ‎ あい‎ あう‎ …‎ をん‎ んあ‎ んい‎ んう
- [`HiraganaIroha`](https://docs.rs/nominals/latest/nominals/struct.HiraganaIroha.html): い‎ ろ‎ は‎ に‎ ほ‎ …‎ す‎ いい‎ いろ‎ いは‎ …‎ せす‎ すい‎ すろ‎ すは
- [`JapaneseFormal`](https://docs.rs/nominals/latest/nominals/struct.JapaneseFormal.html): 零‎ 壱‎ 弐‎ …‎ 九‎ 壱拾‎ 壱拾壱‎ …‎ 壱拾九‎ 弐拾‎ 弐拾壱‎ …‎ 九拾九‎ 壱百‎ 壱百壱
- [`JapaneseInformal`](https://docs.rs/nominals/latest/nominals/struct.JapaneseInformal.html): 〇‎ 一‎ 二‎ …‎ 九‎ 十‎ 十一‎ …‎ 十九‎ 二十‎ 二十一‎ …‎ 九十九‎ 百‎ 百一
- [`Kannada`](https://docs.rs/nominals/latest/nominals/struct.Kannada.html): ೦‎ ೧‎ ೨‎ ೩‎ ೪‎ …‎ ೯‎ ೧೦‎ ೧೧‎ ೧೨‎ …‎ ೯೯‎ ೧೦೦‎ ೧೦೧‎ ೧೦೨
- [`Katakana`](https://docs.rs/nominals/latest/nominals/struct.Katakana.html): ア‎ イ‎ ウ‎ エ‎ オ‎ …‎ ン‎ アア‎ アイ‎ アウ‎ …‎ ヲン‎ ンア‎ ンイ‎ ンウ
- [`KatakanaIroha`](https://docs.rs/nominals/latest/nominals/struct.KatakanaIroha.html): イ‎ ロ‎ ハ‎ ニ‎ ホ‎ …‎ ス‎ イイ‎ イロ‎ イハ‎ …‎ セス‎ スイ‎ スロ‎ スハ
- [`Lao`](https://docs.rs/nominals/latest/nominals/struct.Lao.html): ໐‎ ໑‎ ໒‎ ໓‎ ໔‎ …‎ ໙‎ ໑໐‎ ໑໑‎ ໑໒‎ …‎ ໙໙‎ ໑໐໐‎ ໑໐໑‎ ໑໐໒
- [`Malayalam`](https://docs.rs/nominals/latest/nominals/struct.Malayalam.html): ൦‎ ൧‎ ൨‎ ൩‎ ൪‎ …‎ ൯‎ ൧൦‎ ൧൧‎ ൧൨‎ …‎ ൯൯‎ ൧൦൦‎ ൧൦൧‎ ൧൦൨
- [`Mongolian`](https://docs.rs/nominals/latest/nominals/struct.Mongolian.html): ᠐‎ ᠑‎ ᠒‎ ᠓‎ ᠔‎ …‎ ᠙‎ ᠑᠐‎ ᠑᠑‎ ᠑᠒‎ …‎ ᠙᠙‎ ᠑᠐᠐‎ ᠑᠐᠑‎ ᠑᠐᠒
- [`Myanmar`](https://docs.rs/nominals/latest/nominals/struct.Myanmar.html): ၀‎ ၁‎ ၂‎ ၃‎ ၄‎ …‎ ၉‎ ၁၀‎ ၁၁‎ ၁၂‎ …‎ ၉၉‎ ၁၀၀‎ ၁၀၁‎ ၁၀၂
- [`Oriya`](https://docs.rs/nominals/latest/nominals/struct.Oriya.html): ୦‎ ୧‎ ୨‎ ୩‎ ୪‎ …‎ ୯‎ ୧୦‎ ୧୧‎ ୧୨‎ …‎ ୯୯‎ ୧୦୦‎ ୧୦୧‎ ୧୦୨
- [`Persian`](https://docs.rs/nominals/latest/nominals/struct.Persian.html): ۰‎ ۱‎ ۲‎ ۳‎ ۴‎ …‎ ۹‎ ۱۰‎ ۱۱‎ ۱۲‎ …‎ ۹۹‎ ۱۰۰‎ ۱۰۱‎ ۱۰۲
- [`Tamil`](https://docs.rs/nominals/latest/nominals/struct.Tamil.html): ௦‎ ௧‎ ௨‎ ௩‎ ௪‎ …‎ ௯‎ ௧௦‎ ௧௧‎ ௧௨‎ …‎ ௯௯‎ ௧௦௦‎ ௧௦௧‎ ௧௦௨
- [`Telugu`](https://docs.rs/nominals/latest/nominals/struct.Telugu.html): ౦‎ ౧‎ ౨‎ ౩‎ ౪‎ …‎ ౯‎ ౧౦‎ ౧౧‎ ౧౨‎ …‎ ౯౯‎ ౧౦౦‎ ౧౦౧‎ ౧౦౨
- [`Thai`](https://docs.rs/nominals/latest/nominals/struct.Thai.html): ๐‎ ๑‎ ๒‎ ๓‎ ๔‎ …‎ ๙‎ ๑๐‎ ๑๑‎ ๑๒‎ …‎ ๙๙‎ ๑๐๐‎ ๑๐๑‎ ๑๐๒
- [`Tibetan`](https://docs.rs/nominals/latest/nominals/struct.Tibetan.html): ༠‎ ༡‎ ༢‎ ༣‎ ༤‎ …‎ ༩‎ ༡༠‎ ༡༡‎ ༡༢‎ …‎ ༩༩‎ ༡༠༠‎ ༡༠༡‎ ༡༠༢

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), is open-source.
This repository is available under the [MIT License](./LICENSE-MIT) or the
[Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
