use crate::alphabet::Utf8Letter;

#[derive(Debug, Default)]
pub struct InflectionBuf {
    buf: Vec<u8>,
    stem_len: usize,
}

impl InflectionBuf {
    pub fn new() -> Self {
        Self { buf: Vec::new(), stem_len: 0 }
    }
    pub fn from_stem(stem: &str) -> Self {
        // Reserve extra 8 chars (16 bytes) for string manipulations
        let mut buf = Vec::with_capacity(stem.len() + 16);
        buf.extend_from_slice(stem.as_bytes());
        Self { buf, stem_len: stem.len() }
    }

    pub fn stem_and_ending(&self) -> (&[Utf8Letter], &[Utf8Letter]) {
        let (stem, ending) = unsafe { self.buf.split_at_unchecked(self.stem_len) };
        unsafe { (Utf8Letter::cast_slice(stem), Utf8Letter::cast_slice(ending)) }
    }
    pub fn stem_and_ending_mut(&mut self) -> (&mut [Utf8Letter], &mut [Utf8Letter]) {
        let (stem, ending) = unsafe { self.buf.split_at_mut_unchecked(self.stem_len) };
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

    pub fn append_to_ending(&mut self, append: &str) {
        self.buf.extend_from_slice(append.as_bytes());
    }
    pub fn replace_ending(&mut self, replace: &str) {
        self.buf.splice(self.stem_len.., replace.bytes());
    }

    pub fn append_to_stem(&mut self, insert: &str) {
        self.buf.splice(self.stem_len..self.stem_len, insert.bytes());
        self.stem_len += insert.len();
    }
    pub fn shrink_stem_by(&mut self, shrink_chars: usize) {
        let prev_len = self.stem_len;
        self.stem_len -= shrink_chars * 2;
        self.buf.splice(self.stem_len..prev_len, []);
    }
    pub fn insert_between_last_two_stem_chars(&mut self, insert: &str) {
        let pos = self.stem_len - 2;
        self.buf.splice(pos..pos, insert.bytes());
        self.stem_len += insert.len();
    }
    pub fn remove_pre_last_stem_char(&mut self) {
        self.buf.splice((self.stem_len - 4)..(self.stem_len - 2), []);
        self.stem_len -= 2;
    }
    pub fn remove_stem_char_at(&mut self, char_index: usize) {
        let byte_index = char_index * 2;
        self.buf.splice(byte_index..(byte_index + 2), []);
        self.stem_len -= 2;
    }

    pub fn finish(self) -> String {
        unsafe { String::from_utf8_unchecked(self.buf) }
    }
    pub fn finish_utf16(self) -> Vec<u16> {
        let (buf, len, cap) = self.buf.into_raw_parts();
        let mut utf16: Vec<u16> = unsafe { Vec::from_raw_parts(buf.cast(), len / 2, cap / 2) };

        for n in utf16.iter_mut() {
            *n = unsafe { Utf8Letter::from_utf8_unchecked(n.to_ne_bytes()) }.to_codepoint();
        }
        utf16
    }
}
