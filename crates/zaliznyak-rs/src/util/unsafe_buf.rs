pub(crate) struct UnsafeBuf<'a, const N: usize> {
    start: &'a mut u8,
    end: *mut u8,
}

impl<'a, const N: usize> UnsafeBuf<'a, N> {
    pub const fn new(dst: &'a mut [u8; N]) -> Self {
        let first = dst.as_mut_ptr();
        Self { start: unsafe { &mut *first }, end: first }
    }

    pub const fn len(&self) -> usize {
        unsafe { self.end.offset_from_unsigned(self.start) }
    }
    pub const fn capacity(&self) -> usize {
        N
    }

    pub const fn forward(&mut self, dist: usize) {
        // Check that the move distance is valid
        debug_assert!(self.len() + dist <= N);

        self.end = unsafe { self.end.add(dist) };
    }
    pub const fn chunk<const K: usize>(&mut self) -> &'a mut [u8; K] {
        unsafe { &mut *self.end.cast_array() }
    }

    pub const fn push_str(&mut self, s: &str) {
        unsafe { std::ptr::copy_nonoverlapping(s.as_ptr(), self.end, s.len()) };
        self.forward(s.len());
    }
    pub const fn push(&mut self, ch: char) {
        let buf: &mut [u8; 4] = self.chunk();
        self.forward(ch.encode_utf8(buf).len());
    }

    pub const fn push_fmt<const K: usize>(
        &mut self,
        fmt: impl [const] FnOnce(&mut [u8; K]) -> &mut str,
    ) {
        let len = fmt(self.chunk()).len();
        self.forward(len);
    }
    pub const fn push_fmt2<T: std::marker::Destruct, const K: usize>(
        &mut self,
        value: T,
        fmt: impl [const] FnOnce(T, &mut [u8; K]) -> &mut str,
    ) {
        let len = fmt(value, self.chunk()).len();
        self.forward(len);
    }

    pub const fn finish(self) -> &'a mut str {
        let start = (&raw const *self.start).cast_mut();
        unsafe { str::from_utf8_unchecked_mut(std::slice::from_raw_parts_mut(start, self.len())) }
    }
}
