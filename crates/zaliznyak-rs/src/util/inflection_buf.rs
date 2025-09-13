use crate::alphabet::Utf8Letter;

pub(crate) struct InflectionBuf<'a> {
    start: &'a mut Utf8Letter,
    pub(crate) stem_len: usize,
    pub(crate) len: usize,
}

impl<'a> InflectionBuf<'a> {
    pub const fn max_char_len_for_noun(stem_len: usize) -> usize {
        stem_len / 2 + 5
    }

    pub const fn with_stem_in(stem: &str, buf: &'a mut [Utf8Letter]) -> Self {
        let required_len = Self::max_char_len_for_noun(stem.len());
        assert!(buf.len() >= required_len);
        let buf = buf.as_mut_ptr();

        unsafe { std::ptr::copy_nonoverlapping(stem.as_ptr(), buf.cast(), stem.len()) };
        Self { start: unsafe { &mut *buf }, len: stem.len(), stem_len: stem.len() }
    }

    pub const fn as_slice(&self) -> &'a [Utf8Letter] {
        unsafe { std::slice::from_raw_parts(self.start, self.len / 2) }
    }
    pub const fn as_mut_slice(&mut self) -> &'a mut [Utf8Letter] {
        unsafe { std::slice::from_raw_parts_mut(self.start, self.len / 2) }
    }

    pub const fn stem_and_ending(&self) -> (&'a [Utf8Letter], &'a [Utf8Letter]) {
        unsafe { self.as_slice().split_at_unchecked(self.stem_len / 2) }
    }
    pub const fn stem_and_ending_mut(&mut self) -> (&'a mut [Utf8Letter], &'a mut [Utf8Letter]) {
        let stem_len = self.stem_len;
        unsafe { self.as_mut_slice().split_at_mut_unchecked(stem_len / 2) }
    }

    pub const fn stem(&self) -> &'a [Utf8Letter] {
        self.stem_and_ending().0
    }
    pub const fn stem_mut(&mut self) -> &'a mut [Utf8Letter] {
        self.stem_and_ending_mut().0
    }
    pub const fn ending(&self) -> &'a [Utf8Letter] {
        self.stem_and_ending().1
    }
    pub const fn ending_mut(&mut self) -> &'a mut [Utf8Letter] {
        self.stem_and_ending_mut().1
    }

    fn copy_within(&mut self, from: usize, to: usize, len: usize) {
        let start = (&raw mut *self.start).cast::<u8>();
        unsafe { std::ptr::copy(start.add(from), start.add(to), len) };
    }
    fn copy_into(&mut self, into: usize, s: &[u8]) {
        let start = (&raw mut *self.start).cast::<u8>();
        unsafe { std::ptr::copy_nonoverlapping(s.as_ptr(), start.add(into).cast(), s.len()) }
    }

    pub fn append_to_ending(&mut self, append: &str) {
        self.copy_into(self.len, append.as_bytes());
        self.len += append.len();
    }
    pub fn replace_ending(&mut self, replace: &str) {
        self.copy_into(self.stem_len, replace.as_bytes());
        self.len = self.stem_len + replace.len();
    }

    pub fn append_to_stem(&mut self, insert: &str) {
        self.copy_within(self.stem_len, self.stem_len + insert.len(), self.len - self.stem_len);
        self.copy_into(self.stem_len, insert.as_bytes());
        self.stem_len += insert.len();
        self.len += insert.len();
    }
    pub fn shrink_stem_by(&mut self, shrink_chars: usize) {
        let shrink_len = shrink_chars * 2;
        self.copy_within(self.stem_len, self.stem_len - shrink_len, self.len - self.stem_len);
        self.stem_len -= shrink_len;
        self.len -= shrink_len;
    }
    pub fn insert_between_last_two_stem_chars(&mut self, insert: &str) {
        let pos = self.stem_len - 2;
        self.copy_within(pos, pos + insert.len(), self.len - pos);
        self.copy_into(pos, insert.as_bytes());
        self.stem_len += insert.len();
        self.len += insert.len();
    }
    pub fn remove_pre_last_stem_char(&mut self) {
        self.copy_within(self.stem_len - 2, self.stem_len - 4, self.len - self.stem_len + 2);
        self.stem_len -= 2;
        self.len -= 2;
    }
    pub fn remove_stem_char_at(&mut self, char_index: usize) {
        let char_pos = char_index * 2;
        self.copy_within(char_pos + 2, char_pos, self.len - char_pos - 2);
        self.stem_len -= 2;
        self.len -= 2;
    }

    pub const fn finish(self) -> &'a mut [Utf8Letter] {
        unsafe { std::slice::from_raw_parts_mut(&raw mut *self.start, self.len / 2) }
    }
}
