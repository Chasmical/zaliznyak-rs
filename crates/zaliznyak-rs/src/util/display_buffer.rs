use std::{fmt, mem::MaybeUninit};

pub(crate) struct DisplayBuffer<const N: usize> {
    buf: [MaybeUninit<u8>; N],
    len: usize,
}

impl<const N: usize> DisplayBuffer<N> {
    #[must_use]
    pub const fn new() -> Self {
        Self { buf: [MaybeUninit::uninit(); N], len: 0 }
    }
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.buf.get_unchecked(..self.len).assume_init_ref()) }
    }
}

impl<const N: usize> fmt::Write for DisplayBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(dst) = self.buf.get_mut(self.len..(self.len + s.len())) {
            dst.write_copy_of_slice(s.as_bytes());
            self.len += s.len();
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}
