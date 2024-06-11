//! Generates previews of digit collections in a markdown-compatible table
//! format.

use std::any::type_name;
use std::fs::{File, OpenOptions};
use std::io::{stdout, Write};
use std::path::Path;

use nominals::{
    ArmenianLower, ArmenianUpper, Bengali, Cambodian, CircledNumber, CjkDecimal, CjkEarthlyBranch,
    CjkHeavenlyStem, Decimal, Devanagari, DigitCollection, DoubleCircledNumber, EasternArabic,
    Ethiopic, Georgian, GreekLower, GreekUpper, Gujarati, Gurmukhi, HangeulFormal,
    HangeulJamo, HangeulSyllable, HanjaFormal, HanjaInformal, Hebrew, HexLower, HexUpper, Hiragana,
    HiraganaIroha, JapaneseFormal, JapaneseInformal, Kannada, Katakana, KatakanaIroha, Lao,
    LetterLower, LetterUpper, Malayalam, Mongolian, Myanmar, Nominal, NominalString, NominalSystem,
    Oriya, Persian, RomanLower, RomanUpper, SimplifiedChineseFormal, SimplifiedChineseInformal,
    Tamil, Telugu, Thai, Tibetan, TraditionalChineseFormal, TraditionalChineseInformal,
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() {
    let mut file = open_preview_file_if_saving("index");
    for (name, summary) in vec![
        preview(&DigitPreview(Decimal)),
        preview(&DigitPreview(LetterLower)),
        preview(&DigitPreview(LetterUpper)),
        preview(&RomanLower),
        preview(&RomanUpper),
        preview(&CircledNumber),
        preview(&DoubleCircledNumber),
        preview(&ArmenianLower),
        preview(&ArmenianUpper),
        preview(&DigitPreview(Bengali)),
        preview(&DigitPreview(Cambodian)),
        preview(&SimplifiedChineseInformal::default()),
        preview(&TraditionalChineseInformal::default()),
        preview(&SimplifiedChineseFormal::default()),
        preview(&TraditionalChineseFormal::default()),
        preview(&DigitPreview(CjkDecimal)),
        preview(&DigitPreview(CjkEarthlyBranch)),
        preview(&DigitPreview(CjkHeavenlyStem)),
        preview(&DigitPreview(Devanagari)),
        preview(&DigitPreview(EasternArabic)),
        preview(&Ethiopic),
        preview(&Georgian),
        preview(&DigitPreview(GreekLower)),
        preview(&DigitPreview(GreekUpper)),
        preview(&DigitPreview(Gujarati)),
        preview(&DigitPreview(Gurmukhi)),
        preview(&HanjaInformal),
        preview(&HangeulFormal),
        preview(&DigitPreview(HangeulJamo)),
        preview(&DigitPreview(HangeulSyllable)),
        preview(&HanjaFormal),
        preview(&Hebrew),
        preview(&DigitPreview(HexLower)),
        preview(&DigitPreview(HexUpper)),
        preview(&DigitPreview(Hiragana)),
        preview(&DigitPreview(HiraganaIroha)),
        preview(&JapaneseFormal),
        preview(&JapaneseInformal),
        preview(&DigitPreview(Kannada)),
        preview(&DigitPreview(Katakana)),
        preview(&DigitPreview(KatakanaIroha)),
        preview(&DigitPreview(Lao)),
        preview(&DigitPreview(Malayalam)),
        preview(&DigitPreview(Mongolian)),
        preview(&DigitPreview(Myanmar)),
        preview(&DigitPreview(Oriya)),
        preview(&DigitPreview(Persian)),
        preview(&DigitPreview(Tamil)),
        preview(&DigitPreview(Telugu)),
        preview(&DigitPreview(Thai)),
        preview(&DigitPreview(Tibetan)),
    ] {
        print(file.as_mut(), format_args!("- {name}: {summary}\n"));
    }
}

fn open_preview_file_if_saving(name: &str) -> Option<File> {
    let previews = Path::new("src").join("previews");
    std::fs::create_dir_all(&previews).expect("could not create previews directory");
    std::env::var("SAVE").is_ok().then(|| {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(previews.join(format!("{name}.md")))
            .expect("error opening file")
    })
}

fn preview<System>(system: &System) -> (String, String)
where
    System: Previewable + 'static,
{
    let name = last_type_name::<System>();
    let mut file = open_preview_file_if_saving(name);
    let mut results = Vec::new();
    let mut max_decimal_width = "#".len();
    let mut nominal_pad = name.len();
    let mut previous_value = None;
    for value in system.preview_values() {
        if previous_value.map_or(false, |previous: u32| previous + 1 != value) {
            // This entry is a gap from the previous one
            results.push((NominalString::from('…'), NominalString::from('…')));
        }
        let decimal = value.to_nominal(&Decimal);
        max_decimal_width = max_decimal_width.max(decimal.len());
        let nominal = system.format_nominal(value);
        let unicode_width = nominal
            .graphemes(true)
            .map(|g| g.width().min(2))
            .sum::<usize>();
        nominal_pad = nominal_pad.max(unicode_width);

        results.push((decimal, nominal));
        previous_value = Some(value);
    }

    if let Some(file) = &mut file {
        write!(file, "# {name}\n\n").expect("error writing header");
    }

    let header_pad = if name.len() + 4 == nominal_pad {
        name.len()
    } else {
        nominal_pad
    };

    print(
        file.as_mut(),
        format_args!("| {:max_decimal_width$} | {name:header_pad$} |\n", "#",),
    );
    print(
        file.as_mut(),
        format_args!("|-{:-^max_decimal_width$}-|-{:-^nominal_pad$}-|\n", "", "",),
    );
    let mut one_line = String::new();
    for (decimal, nominal) in results {
        if !one_line.is_empty() {
            one_line.push_str("\u{200E} ");
        }
        one_line.push_str(&nominal);

        let chars = nominal.chars().count();
        let unicode_width = nominal
            .graphemes(true)
            .map(|g| g.width().min(2))
            .sum::<usize>();
        let nominal_pad = nominal_pad + chars - unicode_width;
        print(
            file.as_mut(),
            format_args!("| {decimal:max_decimal_width$} | {nominal:nominal_pad$} |\n"),
        );
    }
    println!();
    (system.doc_link(), one_line)
}

fn print(file: Option<&mut File>, format: std::fmt::Arguments<'_>) {
    stdout()
        .write_fmt(format)
        .expect("error outputing to stdout");
    if let Some(file) = file {
        file.write_fmt(format).expect("error writing to file");
    }
}

fn last_type_name<T>() -> &'static str
where
    T: 'static,
{
    let name = type_name::<T>();
    name.rsplit_once("::")
        .map_or(name, |(_, name)| name.strip_suffix('>').unwrap_or(name))
}

fn markdown_link(name: &str) -> String {
    markdown_link_to(name, name)
}

fn markdown_link_to(name: &str, anchor: &str) -> String {
    format!("[`{name}`](${anchor}$)")
}

trait Previewable: NominalSystem<u32> + Sized + 'static {
    fn doc_link(&self) -> String {
        markdown_link(last_type_name::<Self>())
    }
    fn preview_values(&self) -> Vec<u32>;
}

struct DigitPreview<T>(T);

impl<T> Previewable for DigitPreview<T>
where
    T: DigitCollection + 'static,
{
    fn doc_link(&self) -> String {
        markdown_link(last_type_name::<T>())
    }

    fn preview_values(&self) -> Vec<u32> {
        let count = u32::try_from(self.0.len()).expect("too many digits");

        let mut values = Vec::new();
        let base = u32::from(!(self.0.has_zero_digit() || self.0.zero_based()));

        values.push(base);
        let iters = count.min(5);
        for i in 1..iters {
            values.push(base + i);
        }

        if iters != count {
            values.push(base + count - 1);
        }
        values.push(base + count);
        values.push(base + count + 1);
        values.push(base + count + 2);

        // Show the hundreds transition
        let hundred = count * count;
        values.push(base + hundred - 1);
        values.push(base + hundred);
        values.push(base + hundred + 1);
        values.push(base + hundred + 2);

        values
    }
}

impl<T> NominalSystem<u32> for DigitPreview<T>
where
    T: NominalSystem<u32>,
{
    fn try_format_nominal(
        &self,
        nominal: u32,
    ) -> Result<nominals::NominalString, nominals::Error<u32>> {
        self.0.try_format_nominal(nominal)
    }
}

fn roman_values() -> Vec<u32> {
    vec![
        1, 2, 3, 4, 5, 6, 9, 10, 11, 3_999, 4_000, 4_001, 999_999, 1_000_000, 1_000_001,
    ]
}

fn additive_values() -> Vec<u32> {
    vec![1, 2, 3, 9, 10, 11, 12, 99, 100, 101]
}

impl Previewable for Hebrew {
    fn preview_values(&self) -> Vec<u32> {
        vec![1, 2, 3, 9, 10, 13, 14, 15, 16, 17, 396, 397, 398, 399, 400]
    }
}

impl Previewable for RomanLower {
    fn preview_values(&self) -> Vec<u32> {
        roman_values()
    }
}

impl Previewable for RomanUpper {
    fn preview_values(&self) -> Vec<u32> {
        roman_values()
    }
}

impl Previewable for SimplifiedChineseInformal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for TraditionalChineseInformal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for SimplifiedChineseFormal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for TraditionalChineseFormal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

fn cjk_values() -> Vec<u32> {
    vec![0, 1, 2, 9, 10, 11, 19, 20, 21, 99, 100, 101]
}

impl Previewable for HangeulFormal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for HanjaInformal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for JapaneseFormal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for JapaneseInformal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for ArmenianLower {
    fn preview_values(&self) -> Vec<u32> {
        additive_values()
    }
}

impl Previewable for ArmenianUpper {
    fn preview_values(&self) -> Vec<u32> {
        additive_values()
    }
}

impl Previewable for Georgian {
    fn preview_values(&self) -> Vec<u32> {
        additive_values()
    }
}

impl Previewable for HanjaFormal {
    fn preview_values(&self) -> Vec<u32> {
        cjk_values()
    }
}

impl Previewable for Ethiopic {
    fn preview_values(&self) -> Vec<u32> {
        vec![1, 2, 3, 10, 11, 12, 99, 100, 101, 999, 1000, 1001]
    }
}

impl Previewable for CircledNumber {
    fn preview_values(&self) -> Vec<u32> {
        vec![0, 1, 2, 3, 4, 9, 10, 11, 12, 48, 49, 50]
    }
}

impl Previewable for DoubleCircledNumber {
    fn preview_values(&self) -> Vec<u32> {
        (1..=10).collect()
    }
}
