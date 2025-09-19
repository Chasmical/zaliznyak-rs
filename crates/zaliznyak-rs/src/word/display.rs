use crate::word::{Utf8LetterSlice, Word, WordBuf, find_implicit_insert_stress_pos};
use std::fmt::{self, Write};

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Accent(u32);

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum AccentMode {
    #[default]
    None,
    Explicit,
    Implicit,
}

impl Accent {
    pub const GRAVE: char = '\u{0300}';
    pub const ACUTE: char = '\u{0301}';

    #[must_use]
    pub const fn new(mode: AccentMode, ch: char) -> Self {
        let ch = if mode == AccentMode::None { 0 } else { ch as u32 };
        Self((ch << 8) | mode as u32)
    }
    #[must_use]
    pub const fn mode(&self) -> AccentMode {
        unsafe { std::mem::transmute(self.0 as u8) }
    }
    #[must_use]
    pub const fn char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.0 >> 8) }
    }

    #[must_use]
    pub const fn none() -> Self {
        Self::new(AccentMode::None, '\0')
    }
    #[must_use]
    pub const fn explicit(ch: char) -> Self {
        Self::new(AccentMode::Explicit, ch)
    }
    #[must_use]
    pub const fn implicit(ch: char) -> Self {
        Self::new(AccentMode::Implicit, ch)
    }
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Display<'a> {
    word: Word<'a>,
    accent: Accent,
    ending_sep: Option<char>,
}

impl<'a> Display<'a> {
    #[must_use]
    pub const fn new(word: Word<'a>, accent: Accent, ending_sep: Option<char>) -> Self {
        Self { word, accent, ending_sep }
    }
    #[must_use]
    pub const fn default_display(word: Word<'a>, alternate: bool) -> Self {
        let accent = if alternate { Accent::GRAVE } else { Accent::ACUTE };
        Self::new(word, Accent::implicit(accent), None)
    }
    #[must_use]
    pub const fn default_debug(word: Word<'a>, alternate: bool) -> Self {
        let accent = if alternate { Accent::GRAVE } else { Accent::ACUTE };
        Self::new(word, Accent::explicit(accent), Some('-'))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn accent(self, accent: Accent) -> Self {
        Self { accent, ..self }
    }
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn ending_separator(self, ending_sep: Option<char>) -> Self {
        Self { ending_sep, ..self }
    }

    #[must_use]
    pub const fn get_accent(&self) -> Accent {
        self.accent
    }
    #[must_use]
    pub const fn get_ending_sep(&self) -> Option<char> {
        self.ending_sep
    }
}

impl<'a> Word<'a> {
    #[must_use = "this does not display the word, it returns an object that can be displayed"]
    pub const fn display(self) -> Display<'a> {
        Display::default_display(self, false)
    }
}
impl WordBuf {
    #[must_use = "this does not display the word, it returns an object that can be displayed"]
    pub const fn display(&self) -> Display<'_> {
        Display::default_display(self.borrow(), false)
    }
}

impl fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::default_display(*self, f.alternate()).fmt(f)
    }
}
impl fmt::Display for WordBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::default_display(self.borrow(), f.alternate()).fmt(f)
    }
}
impl fmt::Debug for Word<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&Display::default_debug(*self, f.alternate()), f)
    }
}
impl fmt::Debug for WordBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&Display::default_debug(self.borrow(), f.alternate()), f)
    }
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.word.info;

        let add_accent = match self.accent.mode() {
            AccentMode::None => false,
            AccentMode::Explicit => info.insert_stress_pos > 0,
            AccentMode::Implicit => {
                let implicit_pos = find_implicit_insert_stress_pos(self.word.as_letters());
                implicit_pos != Some(info.insert_stress_pos)
            },
        };

        if add_accent && info.insert_stress_pos <= info.stem_len {
            let (stem1, stem2) = self.word.stem_letters().split_at(info.insert_stress_pos);
            f.write_str(stem1.as_str())?;
            f.write_char(self.accent.char())?;
            f.write_str(stem2.as_str())?;
        } else {
            f.write_str(self.word.stem())?;
        }

        if let Some(ending_sep) = self.ending_sep
            && info.stem_len != self.word.buf.len()
        {
            f.write_char(ending_sep)?;
        }
        if add_accent && info.insert_stress_pos > info.stem_len {
            let pos = info.insert_stress_pos - info.stem_len;
            let (ending1, ending2) = self.word.ending_letters().split_at(pos);
            f.write_str(ending1.as_str())?;
            f.write_char(self.accent.char())?;
            f.write_str(ending2.as_str())?;
        } else {
            f.write_str(self.word.ending())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::{Utf8Letter::*, WordInfo};

    #[test]
    fn fmt() {
        // Display indicates stress only if it can't be inferred automatically
        //   and uses the acute accent by default, without the ending separator.
        assert_eq!(
            format!("{}", WordBuf {
                buf: [Я, Б, Л, О, К, О].into(),
                info: WordInfo { stem_len: 5, insert_stress_pos: 1 },
            }),
            "я́блоко",
        );
        assert_eq!(
            format!("{}", WordBuf {
                buf: [С, Е, С, Т, Ё, Р].into(),
                info: WordInfo { stem_len: 6, insert_stress_pos: 5 },
            }),
            "сестёр",
        );
        assert_eq!(
            format!("{}", WordBuf {
                buf: [Р, О, Д].into(),
                info: WordInfo { stem_len: 3, insert_stress_pos: 2 },
            }),
            "род",
        );

        // Debug always indicates stress (even on 'ё') and uses the acute accent by
        //   default, and also separates the non-empty ending from the stem with '-'.
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [Ш, Е, С, Т, Е, Р, Н, Я].into(),
                info: WordInfo { stem_len: 7, insert_stress_pos: 8 },
            }),
            "шестерн-я́",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [С, Е, С, Т, Ё, Р].into(),
                info: WordInfo { stem_len: 6, insert_stress_pos: 5 },
            }),
            "сестё́р",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [Р, О, Д].into(),
                info: WordInfo { stem_len: 3, insert_stress_pos: 2 },
            }),
            "ро́д",
        );

        // Alternate formatting mode ({:#} or {:#?}) uses grave accent instead of acute.
        assert_eq!(
            format!("{:#}", WordBuf {
                buf: [Г, Р, У, Ш, А].into(),
                info: WordInfo { stem_len: 4, insert_stress_pos: 3 },
            }),
            "гру̀ша",
        );
        assert_eq!(
            format!("{:#?}", WordBuf {
                buf: [Г, Р, У, Ш, А].into(),
                info: WordInfo { stem_len: 4, insert_stress_pos: 3 },
            }),
            "гру̀ш-а",
        );

        // Letter 'ё' always receives stress, unless explicitly specified otherwise.
        // Debug always indicates stress, even on 'ё', for maximum clarity.
        assert_eq!(
            format!("{}", WordBuf {
                buf: [С, Ё, Р, А].into(),
                info: WordInfo { stem_len: 3, insert_stress_pos: 4 },
            }),
            "сёра́",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [С, Ё, Р, А].into(),
                info: WordInfo { stem_len: 3, insert_stress_pos: 4 },
            }),
            "сёр-а́",
        );
    }
}
