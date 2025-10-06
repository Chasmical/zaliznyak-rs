use crate::word::Utf8Letter;

pub(crate) struct UnsafeParser<'a> {
    current: &'a u8,
    end: &'a u8,
}

impl<'a> UnsafeParser<'a> {
    pub const fn new(s: &'a str) -> Self {
        let r = s.as_bytes().as_ptr_range();
        unsafe { Self { current: &*r.start, end: &*r.end } }
    }

    pub const fn remaining_len(&self) -> usize {
        unsafe { (&raw const *self.end).offset_from_unsigned(self.current) }
    }
    pub const fn remaining(&self) -> &'a [u8] {
        unsafe { std::slice::from_ptr_range(self.current..self.end) }
    }
    pub const fn remaining_str(&self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.remaining()) }
    }

    pub const fn forward(&mut self, dist: usize) {
        // Check that the move distance is valid
        debug_assert!(dist <= self.remaining_len());

        self.current = unsafe { &*(&raw const *self.current).add(dist) };

        // Check that the next byte is not a UTF-8 continuation byte
        debug_assert!(!matches!(self.peek_one().unwrap_or(&0), 0x80..=0xBF));
    }
    pub const fn finished(&self) -> bool {
        self.remaining_len() == 0
    }

    pub const fn peek<const N: usize>(&self) -> Option<&'a [u8; N]> {
        self.remaining().first_chunk::<N>()
    }
    pub const fn peek_one(&self) -> Option<&'a u8> {
        if !self.finished() { Some(self.current) } else { None }
    }
    pub fn peek_char(&self) -> Option<char> {
        self.remaining_str().chars().next()
    }
    pub const fn peek_letter(&self) -> Option<Utf8Letter> {
        match self.peek::<2>() {
            Some(bytes) => Utf8Letter::from_utf8(*bytes),
            None => None,
        }
    }

    pub const fn read<const N: usize>(&mut self) -> Option<&'a [u8; N]> {
        if let Some(chunk) = self.remaining().first_chunk::<N>() {
            self.forward(N);
            return Some(chunk);
        }
        None
    }
    pub const fn read_one(&mut self) -> Option<&'a u8> {
        if !self.finished() {
            let read = self.current;
            self.forward(1);
            return Some(read);
        }
        None
    }
    pub fn read_char(&mut self) -> Option<char> {
        if let Some(read) = self.peek_char() {
            self.forward(read.len_utf8());
            return Some(read);
        }
        None
    }

    pub const fn skip_bytes(&mut self, bytes: &[u8]) -> bool {
        // FIXME(const-hack): Replace with `self.remaining().starts_with(bytes)`.
        if self.remaining_len() >= bytes.len() {
            let peeked = unsafe { std::slice::from_raw_parts(self.current, bytes.len()) };
            if peeked == bytes {
                self.forward(bytes.len());
                return true;
            }
        }
        false
    }
    pub const fn skip_str(&mut self, s: &str) -> bool {
        self.skip_bytes(s.as_bytes())
    }
    pub const fn skip(&mut self, ch: char) -> bool {
        self.skip_str(ch.encode_utf8(&mut [0; 4]))
    }
    pub fn skip_char(&mut self) -> bool {
        self.peek_char().map(|x| self.forward(x.len_utf8())).is_some()
    }
}

pub(crate) const trait PartialFromStr: std::str::FromStr + Sized {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err>;

    fn from_str_or_err(s: &str, default_err: Self::Err) -> Result<Self, Self::Err>
    where
        Self::Err: [const] std::marker::Destruct,
        Result<Self, Self::Err>: [const] std::marker::Destruct,
    {
        let mut parser = UnsafeParser::new(s);

        match Self::partial_from_str(&mut parser) {
            Ok(result) if parser.finished() => Ok(result),
            Err(err) => Err(err),
            _ => Err(default_err),
        }
    }
}
