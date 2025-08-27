use crate::alphabet::Utf8Letter;

pub(crate) const INFLECTION_MAX_EXTRA_LEN: usize = 5 * 2;

#[derive(Debug)]
pub struct InflectionBuf<'a> {
    start: &'a mut u8,
    len: usize,
    stem_len: usize,
}

impl<'a> InflectionBuf<'a> {
    pub fn new(stem: &str, buf: &'a mut [u8]) -> Self {
        let required_len = stem.len() + INFLECTION_MAX_EXTRA_LEN;
        assert!(buf.len() >= required_len);
        unsafe { std::ptr::copy_nonoverlapping(stem.as_ptr(), buf.as_mut_ptr(), stem.len()) };

        Self { start: unsafe { &mut *buf.as_mut_ptr() }, len: stem.len(), stem_len: stem.len() }
    }

    pub fn stem_and_ending(&self) -> (&[Utf8Letter], &[Utf8Letter]) {
        let slice = unsafe { std::slice::from_raw_parts(self.start, self.len) };
        let (stem, ending) = unsafe { slice.split_at_unchecked(self.stem_len) };
        unsafe { (Utf8Letter::cast_slice(stem), Utf8Letter::cast_slice(ending)) }
    }
    pub fn stem_and_ending_mut(&mut self) -> (&mut [Utf8Letter], &mut [Utf8Letter]) {
        let slice = unsafe { std::slice::from_raw_parts_mut(self.start, self.len) };
        let (stem, ending) = unsafe { slice.split_at_mut_unchecked(self.stem_len) };
        unsafe { (Utf8Letter::cast_slice_mut(stem), Utf8Letter::cast_slice_mut(ending)) }
    }

    pub fn stem(&self) -> &[Utf8Letter] {
        self.stem_and_ending().0
    }
    pub fn stem_mut(&mut self) -> &mut [Utf8Letter] {
        self.stem_and_ending_mut().0
    }
    pub fn ending(&self) -> &[Utf8Letter] {
        self.stem_and_ending().1
    }
    pub fn ending_mut(&mut self) -> &mut [Utf8Letter] {
        self.stem_and_ending_mut().1
    }

    fn copy_within(&mut self, from: usize, to: usize, len: usize) {
        let start = &raw mut *self.start;
        unsafe { std::ptr::copy(start.add(from), start.add(to), len); }
    }
    fn copy_into(&mut self, into: usize, s: &[u8]) {
        let start = &raw mut *self.start;
        unsafe { std::ptr::copy_nonoverlapping(s.as_ptr(), start.add(into), s.len()) }
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

    pub fn finish(self) -> &'a mut str {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.start, self.len);
            str::from_utf8_unchecked_mut(slice)
        }
    }
}
