pub(crate) struct UnsafeBuf<'a> {
    start: &'a u8,
    end: &'a mut u8,
}

impl<'a> UnsafeBuf<'a> {
    pub const fn new<const N: usize>(dst: &'a mut [u8; N]) -> Self {
        let start = dst.as_mut_ptr();
        unsafe { Self { start: &*start, end: &mut *start } }
    }

    pub const fn forward(&mut self, distance: usize) {
        self.end = unsafe { &mut *(self.end as *mut u8).add(distance) };
    }
    pub const fn chunk<const N: usize>(&mut self) -> &'a mut [u8; N] {
        unsafe { &mut *(self.end as *mut u8 as *mut [u8; N]) }
    }
    pub const fn finish(self) -> &'a mut str {
        unsafe {
            let start = self.start as *const u8 as *mut u8;
            let len = (self.end as *mut u8).offset_from_unsigned(start);
            str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(start, len))
        }
    }

    pub const fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.end, bytes.len()) };
        self.forward(bytes.len());
    }
    pub const fn push_str(&mut self, s: &str) {
        self.push_bytes(s.as_bytes());
    }
    pub const fn push(&mut self, ch: char) {
        let buf: &mut [u8; 4] = self.chunk();
        self.forward(ch.encode_utf8(buf).len());
    }
}
