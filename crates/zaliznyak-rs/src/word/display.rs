use crate::word::{Utf8LetterSlice, Word, WordBuf, find_implicit_insert_stress_pos};
use std::fmt::{self, Write};

/// Accent display info, storing [`AccentMode`] and the accent [`char`].
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Accent(u32);

/// Accent display mode.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum AccentMode {
    /// Don't output the stress.
    #[default]
    None,
    /// Always output the stress.
    Explicit,
    /// Output the stress only if it can't be automatically inferred.
    Implicit,
}

impl Accent {
    /// Grave accent char. а̀ѐё̀ѝо̀у̀ы̀э̀ю̀я̀.
    pub const GRAVE: char = '\u{0300}';
    /// Acute accent char. а́е́ё́и́о́у́ы́э́ю́я́.
    pub const ACUTE: char = '\u{0301}';

    /// Constructs a new `Accent` from [`AccentMode`] and [`char`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::{Accent, AccentMode};
    ///
    /// let acc = Accent::new(AccentMode::None, Accent::ACUTE);
    /// assert_eq!(acc.mode(), AccentMode::None);
    /// assert_eq!(acc.char(), '\0');
    ///
    /// let acc = Accent::new(AccentMode::Explicit, Accent::GRAVE);
    /// assert_eq!(acc.mode(), AccentMode::Explicit);
    /// assert_eq!(acc.char(), Accent::GRAVE);
    /// ```
    #[must_use]
    pub const fn new(mode: AccentMode, ch: char) -> Self {
        let ch = if mode == AccentMode::None { 0 } else { ch as u32 };
        Self((ch << 8) | mode as u32)
    }

    /// Constructs a new `Accent` with [`AccentMode::None`].
    #[must_use]
    pub const fn none() -> Self {
        Self::new(AccentMode::None, '\0')
    }
    /// Constructs a new `Accent` with [`AccentMode::Explicit`] and specified char.
    #[must_use]
    pub const fn explicit(ch: char) -> Self {
        Self::new(AccentMode::Explicit, ch)
    }
    /// Constructs a new `Accent` with [`AccentMode::Implicit`] and specified char.
    #[must_use]
    pub const fn implicit(ch: char) -> Self {
        Self::new(AccentMode::Implicit, ch)
    }

    /// Returns the accent's mode.
    #[must_use]
    pub const fn mode(&self) -> AccentMode {
        unsafe { std::mem::transmute(self.0 as u8) }
    }
    /// Returns the accent's char. Returns `'\0'` if the mode is [`AccentMode::None`].
    #[must_use]
    pub const fn char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.0 >> 8) }
    }
}

/// Helper struct for displaying [`Word`] with [`format!`] and `{}`.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Display<'a> {
    word: Word<'a>,
    accent: Accent,
    ending_sep: Option<char>,
}

impl<'a> Display<'a> {
    /// Constructs a new `Display` for the word, with specified display parameters.
    #[must_use]
    pub const fn new(word: Word<'a>, accent: Accent, ending_sep: Option<char>) -> Self {
        Self { word, accent, ending_sep }
    }
    /// Constructs a new `Display` for the word, with default parameters for [`fmt::Display`].
    ///
    /// Default: implicit acute accent а́ (alternate: grave а̀), and no ending separator.
    #[must_use]
    pub const fn default_display(word: Word<'a>, alternate: bool) -> Self {
        let accent = if alternate { Accent::GRAVE } else { Accent::ACUTE };
        Self::new(word, Accent::implicit(accent), None)
    }
    /// Constructs a new `Display` for the word, with default parameters for [`fmt::Debug`].
    ///
    /// Default: explicit acute accent а́ (alternate: grave а̀), and `-` as an ending separator.
    #[must_use]
    pub const fn default_debug(word: Word<'a>, alternate: bool) -> Self {
        let accent = if alternate { Accent::GRAVE } else { Accent::ACUTE };
        Self::new(word, Accent::explicit(accent), Some('-'))
    }

    /// Sets the accent display info.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn accent(self, accent: Accent) -> Self {
        Self { accent, ..self }
    }
    /// Sets or removes the ending separator char.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn ending_separator(self, ending_sep: Option<char>) -> Self {
        Self { ending_sep, ..self }
    }

    /// Returns the current accent display info.
    #[must_use]
    pub const fn get_accent(&self) -> Accent {
        self.accent
    }
    /// Returns the current ending separator char.
    #[must_use]
    pub const fn get_ending_sep(&self) -> Option<char> {
        self.ending_sep
    }
}

impl<'a> Word<'a> {
    /// Returns a configurable object implementing [`fmt::Display`] for displaying this word.
    ///
    /// Consider using one of the pre-defined formats instead:
    ///
    /// ```
    /// use zaliznyak::word::{Word, WordBuf};
    ///
    /// let buf: WordBuf = "сло́в-о".parse().unwrap();
    /// let word: Word = buf.borrow();
    ///
    /// // as_str (just the word itself, no extra info)
    /// assert_eq!(word.as_str(), "слово");
    ///
    /// // Display (omits inferrable stress, and no ending separator)
    /// assert_eq!(format!("{}", word), "сло́во");
    /// assert_eq!(format!("{:#}", word), "сло̀во");
    ///
    /// // Debug (always outputs stress and ending separator)
    /// assert_eq!(format!("{:?}", word), "сло́в-о");
    /// assert_eq!(format!("{:#?}", word), "сло̀в-о");
    /// ```
    #[must_use = "this does not display the word, it returns an object that can be displayed"]
    pub const fn display(self) -> Display<'a> {
        Display::default_display(self, false)
    }
}
impl WordBuf {
    /// Returns a configurable object implementing [`fmt::Display`] for displaying this word.
    ///
    /// Consider using one of the pre-defined formats instead:
    ///
    /// ```
    /// use zaliznyak::word::WordBuf;
    ///
    /// let buf: WordBuf = "сло́в-о".parse().unwrap();
    ///
    /// // as_str (just the word itself, no extra info)
    /// assert_eq!(buf.as_str(), "слово");
    ///
    /// // Display (omits inferrable stress, and no ending separator)
    /// assert_eq!(format!("{}", buf), "сло́во");
    /// assert_eq!(format!("{:#}", buf), "сло̀во");
    ///
    /// // Debug (always outputs stress and ending separator)
    /// assert_eq!(format!("{:?}", buf), "сло́в-о");
    /// assert_eq!(format!("{:#?}", buf), "сло̀в-о");
    /// ```
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
        let add_accent = match self.accent.mode() {
            AccentMode::None => false,
            AccentMode::Explicit => self.word.stress_at > 0,
            AccentMode::Implicit => {
                let implicit_pos = find_implicit_insert_stress_pos(self.word.as_letters());
                implicit_pos != Some(self.word.stress_at)
            },
        };

        if add_accent && self.word.stress_at <= self.word.stem_len {
            let (stem1, stem2) = self.word.stem_letters().split_at(self.word.stress_at);
            f.write_str(stem1.as_str())?;
            f.write_char(self.accent.char())?;
            f.write_str(stem2.as_str())?;
        } else {
            f.write_str(self.word.stem())?;
        }

        if let Some(ending_sep) = self.ending_sep
            && self.word.stem_len != self.word.buf.len()
        {
            f.write_char(ending_sep)?;
        }
        if add_accent && self.word.stress_at > self.word.stem_len {
            let pos = self.word.stress_at - self.word.stem_len;
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
    use crate::word::Utf8Letter::*;

    #[test]
    #[rustfmt::skip]
    fn fmt() {
        // Display indicates stress only if it can't be inferred automatically
        //   and uses the acute accent by default, without the ending separator.
        assert_eq!(
            format!("{}", WordBuf {
                buf: [Я, Б, Л, О, К, О].into(),
                stem_len: 5, stress_at: 1,
            }),
            "я́блоко",
        );
        assert_eq!(
            format!("{}", WordBuf {
                buf: [С, Е, С, Т, Ё, Р].into(),
                stem_len: 6, stress_at: 5,
            }),
            "сестёр",
        );
        assert_eq!(
            format!("{}", WordBuf {
                buf: [Р, О, Д].into(),
                stem_len: 3, stress_at: 2,
            }),
            "род",
        );

        // Debug always indicates stress (even on 'ё') and uses the acute accent by
        //   default, and also separates the non-empty ending from the stem with '-'.
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [Ш, Е, С, Т, Е, Р, Н, Я].into(),
                stem_len: 7, stress_at: 8,
            }),
            "шестерн-я́",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [С, Е, С, Т, Ё, Р].into(),
                stem_len: 6, stress_at: 5,
            }),
            "сестё́р",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [Р, О, Д].into(),
                stem_len: 3, stress_at: 2,
            }),
            "ро́д",
        );

        // Alternate formatting mode ({:#} or {:#?}) uses grave accent instead of acute.
        assert_eq!(
            format!("{:#}", WordBuf {
                buf: [Г, Р, У, Ш, А].into(),
                stem_len: 4, stress_at: 3,
            }),
            "гру̀ша",
        );
        assert_eq!(
            format!("{:#?}", WordBuf {
                buf: [Г, Р, У, Ш, А].into(),
                stem_len: 4, stress_at: 3,
            }),
            "гру̀ш-а",
        );

        // Letter 'ё' always receives stress, unless explicitly specified otherwise.
        // Debug always indicates stress, even on 'ё', for maximum clarity.
        assert_eq!(
            format!("{}", WordBuf {
                buf: [С, Ё, Р, А].into(),
                stem_len: 3, stress_at: 4,
            }),
            "сёра́",
        );
        assert_eq!(
            format!("{:?}", WordBuf {
                buf: [С, Ё, Р, А].into(),
                stem_len: 3, stress_at: 4,
            }),
            "сёр-а́",
        );
    }
}
