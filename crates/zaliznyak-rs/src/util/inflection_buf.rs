use crate::word::{Utf8Letter, WordBuf};

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub(crate) enum StressPos {
    Stem,
    Ending,
}

pub(crate) struct InflectionBuf<'a> {
    ptr: &'a mut Utf8Letter,
    pub(crate) len: usize,
    pub(crate) stem_len: usize,
    pub(crate) stress_at: usize,
    pub(crate) stress: StressPos,
}

impl<'a> InflectionBuf<'a> {
    pub fn new(word: &mut WordBuf) -> Self {
        Self {
            ptr: unsafe { &mut *word.buf.as_mut_ptr() },
            len: word.buf.len(),
            stem_len: word.stem_len,
            stress_at: word.stress_at,
            stress: StressPos::Stem,
        }
    }

    pub const fn is_stem_stressed(&self) -> bool {
        self.stress == StressPos::Stem
    }
    pub const fn is_ending_stressed(&self) -> bool {
        self.stress == StressPos::Ending
    }

    pub const fn as_slice(&self) -> &[Utf8Letter] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [Utf8Letter] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    pub const fn stem_and_ending(&self) -> (&[Utf8Letter], &[Utf8Letter]) {
        unsafe { self.as_slice().split_at_unchecked(self.stem_len) }
    }
    pub const fn stem_and_ending_mut(&mut self) -> (&mut [Utf8Letter], &mut [Utf8Letter]) {
        let stem_len = self.stem_len;
        unsafe { self.as_mut_slice().split_at_mut_unchecked(stem_len) }
    }

    pub const fn stem(&self) -> &[Utf8Letter] {
        self.stem_and_ending().0
    }
    pub const fn stem_mut(&mut self) -> &mut [Utf8Letter] {
        self.stem_and_ending_mut().0
    }
    pub const fn ending(&self) -> &[Utf8Letter] {
        self.stem_and_ending().1
    }
    pub const fn ending_mut(&mut self) -> &mut [Utf8Letter] {
        self.stem_and_ending_mut().1
    }

    pub fn set_stress_at(&mut self, at: &Utf8Letter) {
        self.stress_at = self.as_slice().element_offset(at).unwrap() + 1;
    }

    fn copy_within(&mut self, from: usize, to: usize, len: usize) {
        unsafe {
            let start = &raw mut *self.ptr;
            std::ptr::copy(start.add(from), start.add(to), len);
        }
    }
    fn copy_into(&mut self, into: usize, s: &str) {
        unsafe {
            let start = &raw mut *self.ptr;
            std::ptr::copy_nonoverlapping(s.as_ptr(), start.add(into).cast(), s.len());
        }
    }

    pub fn append_to_ending(&mut self, append: &str) {
        self.copy_into(self.len, append);
        self.len += append.len() / 2;
    }
    pub fn replace_ending(&mut self, replace: &str) {
        self.copy_into(self.stem_len, replace);
        self.len = self.stem_len + replace.len() / 2;
    }

    pub fn append_to_stem(&mut self, insert: &str) {
        let insert_len = insert.len() / 2;
        self.copy_within(self.stem_len, self.stem_len + insert_len, self.len - self.stem_len);
        self.copy_into(self.stem_len, insert);
        self.stem_len += insert_len;
        self.len += insert_len;
    }
    pub fn shrink_stem_by(&mut self, shrink_len: usize) {
        self.copy_within(self.stem_len, self.stem_len - shrink_len, self.len - self.stem_len);
        self.stem_len -= shrink_len;
        self.len -= shrink_len;
    }
    pub fn insert_between_last_two_stem_chars(&mut self, insert: &str) {
        let insert_len = insert.len() / 2;
        let pos = self.stem_len - 1;
        self.copy_within(pos, pos + insert_len, self.len - pos);
        self.copy_into(pos, insert);
        self.stem_len += insert_len;
        self.len += insert_len;
    }
    pub fn remove_pre_last_stem_char(&mut self) {
        self.copy_within(self.stem_len - 1, self.stem_len - 2, self.len - self.stem_len + 1);
        self.stem_len -= 1;
        self.len -= 1;
    }
    pub fn remove_stem_char_at(&mut self, char_pos: usize) {
        self.copy_within(char_pos + 1, char_pos, self.len - char_pos - 1);
        self.stem_len -= 1;
        self.len -= 1;
    }

    pub fn finish(self, word: &mut WordBuf) {
        unsafe { word.buf.set_len(self.len) };
        word.stem_len = self.stem_len;
        word.stress_at = self.stress_at;
    }
}
