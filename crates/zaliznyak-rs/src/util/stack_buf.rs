use crate::alphabet::Utf8Letter;

#[repr(u8)]
pub enum StackBuf<T, const N: usize> {
    Stack([T; N]),
    Heap(Vec<T>),
}

impl<T: Copy, const N: usize> StackBuf<T, N> {
    pub fn new(required_len: usize) -> Self {
        if required_len <= N {
            Self::Stack([unsafe { std::mem::MaybeUninit::uninit().assume_init() }; N])
        } else {
            Self::Heap(Vec::with_capacity(required_len))
        }
    }

    pub const fn as_ptr(&self) -> *const T {
        match self {
            Self::Stack(buf) => buf.as_ptr(),
            Self::Heap(v) => v.as_ptr(),
        }
    }

    pub const fn as_slice(&self) -> &[T] {
        match self {
            Self::Stack(buf) => buf,
            Self::Heap(v) => unsafe { std::slice::from_raw_parts(v.as_ptr(), v.capacity()) },
        }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            Self::Stack(buf) => buf,
            Self::Heap(v) => unsafe {
                std::slice::from_raw_parts_mut(v.as_mut_ptr(), v.capacity())
            },
        }
    }

    pub fn into_vec(self, len: usize) -> Vec<T> {
        match self {
            Self::Stack(buf) => (&buf[..len]).to_vec(),
            Self::Heap(mut v) => {
                unsafe { v.set_len(len) };
                v
            },
        }
    }
}

impl<const N: usize> StackBuf<Utf8Letter, N> {
    pub fn into_string(self, len: usize) -> String {
        let slice = unsafe { std::slice::from_raw_parts(self.as_ptr().cast(), len) };
        unsafe { String::from_utf8_unchecked(slice.to_vec()) }
    }
}
