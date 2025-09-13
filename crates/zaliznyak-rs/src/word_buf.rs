use crate::{
    alphabet::{Utf8Letter, Utf8LetterExt},
    util::{InflectionBuf, StackBuf},
};

type Pos = u8;

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
struct WordInfo {
    len: Pos,
    stem_len: Pos,
    stress_pos: Pos,
}

#[derive_const(Default)]
pub struct WordBuf {
    // Declinable parts of Russian words very rarely exceed 15 letters
    buf: StackBuf<Utf8Letter, 15>,
    info: WordInfo,
}

#[derive(Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Word<'a> {
    buf: &'a [Utf8Letter],
    info: WordInfo,
}

impl WordInfo {
    pub const fn entire<'a>(&self, slice: &'a [Utf8Letter]) -> &'a [Utf8Letter] {
        unsafe { slice.get_unchecked(..self.len as _) }
    }
    pub const fn stem<'a>(&self, slice: &'a [Utf8Letter]) -> &'a [Utf8Letter] {
        unsafe { slice.get_unchecked(..self.stem_len as _) }
    }
    pub const fn ending<'a>(&self, slice: &'a [Utf8Letter]) -> &'a [Utf8Letter] {
        unsafe { slice.get_unchecked(self.stem_len as _..self.len as _) }
    }
}

impl WordBuf {
    pub fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: StackBuf::with_capacity(cap), info: Default::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.info.len == 0
    }
    pub const fn as_str(&self) -> &str {
        self.info.entire(&self.buf).as_str()
    }
    pub const fn stem(&self) -> &str {
        self.info.stem(&self.buf).as_str()
    }
    pub const fn ending(&self) -> &str {
        self.info.ending(&self.buf).as_str()
    }
    pub const fn split_by_stress(&self) -> (&str, &str) {
        let (left, right) = self.info.entire(&self.buf).split_at((self.info.stress_pos + 1) as _);
        (left.as_str(), right.as_str())
    }

    pub const fn borrow(&self) -> Word<'_> {
        Word { buf: &self.buf, info: self.info }
    }

    pub fn inflect<F: FnOnce(&mut [Utf8Letter]) -> Word<'_>>(&mut self, f: F) {
        let word = f(&mut self.buf);
        self.info = word.info;
    }
    pub fn into_string(self) -> String {
        self.buf.into_string(self.info.len as _)
    }
}

impl Word<'_> {
    pub fn is_empty(&self) -> bool {
        self.info.len == 0
    }
    pub const fn as_str(&self) -> &str {
        self.info.entire(self.buf).as_str()
    }
    pub const fn stem(&self) -> &str {
        self.info.stem(self.buf).as_str()
    }
    pub const fn ending(&self) -> &str {
        self.info.ending(self.buf).as_str()
    }
    pub const fn split_by_stress(&self) -> (&str, &str) {
        let (left, right) = self.info.entire(self.buf).split_at((self.info.stress_pos + 1) as _);
        (left.as_str(), right.as_str())
    }

    pub fn to_owned(&self) -> WordBuf {
        WordBuf { buf: StackBuf::copied_from(self.buf), info: self.info }
    }
}

// Implement some basic traits for WordBuf manually, since they can't be auto-derived
impl Clone for WordBuf {
    fn clone(&self) -> Self {
        Self { buf: StackBuf::copied_from(&self.buf), info: self.info }
    }
}
impl PartialEq for WordBuf {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.buf.as_slice() == other.buf.as_slice()
    }
}
impl Eq for WordBuf {}
impl std::hash::Hash for WordBuf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        self.info.entire(&self.buf).hash(state);
    }
}

// Implement Display and Debug traits
impl std::fmt::Display for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn needs_stress_annotation(stress_pos: usize, word: &[Utf8Letter]) -> bool {
            let mut iter = word.iter().copied().enumerate().filter(|x| x.1.is_vowel());
            let mut multiple = false;

            while let Some((i, first)) = iter.next() {
                if i == stress_pos {
                    if first == Utf8Letter::Ё {
                        return false;
                    }
                    return multiple || iter.next().is_some();
                }
                multiple = true;
            }
            return false;
        }

        let (left, right) = self.split_by_stress();

        let needs_stress =
            needs_stress_annotation(self.info.stress_pos as _, self.info.entire(self.buf));

        let accent =
            if needs_stress { if f.alternate() { "\u{0300}" } else { "\u{0301}" } } else { "" };

        write!(f, "{left}{accent}{right}")
    }
}
impl std::fmt::Display for WordBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}
impl std::fmt::Debug for Word<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl std::fmt::Debug for WordBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// TODO: refactor to pass stress_pos
impl<'a> From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        Self {
            info: WordInfo {
                len: (value.len / 2) as _,
                stem_len: (value.stem_len / 2) as _,
                stress_pos: 0,
            },
            buf: value.finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new(stem: &str, ending: &str, stress_pos: usize) -> WordBuf {
        let mut buf = WordBuf::with_capacity_for(stem);

        buf.inflect(|dst| {
            let mut dst = InflectionBuf::with_stem_in(stem, dst);
            dst.append_to_ending(ending);
            dst.into()
        });

        buf.info.stress_pos = stress_pos as _;
        buf
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", new("яблок", "о", 0)), "я́блоко");
        assert_eq!(format!("{:#}", new("яблок", "о", 0)), "я̀блоко");
        assert_eq!(format!("{}", new("груш", "а", 2)), "гру́ша");
        assert_eq!(format!("{:#}", new("груш", "а", 2)), "гру̀ша");
        assert_eq!(format!("{}", new("шестерн", "я", 7)), "шестерня́");
        assert_eq!(format!("{:#}", new("шестерн", "я", 7)), "шестерня̀");

        assert_eq!(format!("{}", new("род", "", 1)), "род");
        assert_eq!(format!("{:#}", new("род", "", 1)), "род");
        assert_eq!(format!("{}", new("сестёр", "", 4)), "сестёр");
        assert_eq!(format!("{:#}", new("сестёр", "", 4)), "сестёр");
        assert_eq!(format!("{}", new("сёр", "а", 3)), "сёра́");
        assert_eq!(format!("{:#}", new("сёр", "а", 3)), "сёра̀");
    }
}
