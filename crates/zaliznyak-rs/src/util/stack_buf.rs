use crate::alphabet::Utf8Letter;

pub(crate) struct StackBuf<T, const N: usize> {
    buf: Buf<T, N>,
}

#[repr(u8)]
enum Buf<T, const N: usize> {
    Stack([T; N]),
    Heap(Vec<T>),
}

impl<T, const N: usize> StackBuf<T, N> {
    #[allow(clippy::uninit_vec, clippy::uninit_assumed_init)]
    pub fn with_capacity(cap: usize) -> Self {
        if cap <= N {
            Self { buf: Buf::Stack(unsafe { std::mem::MaybeUninit::uninit().assume_init() }) }
        } else {
            let mut vec = Vec::with_capacity(cap);
            unsafe { vec.set_len(cap) };
            Self { buf: Buf::Heap(vec) }
        }
    }
    pub fn from(value: &[T]) -> Self
    where T: Copy {
        let mut buf = Self::with_capacity(value.len());
        buf.as_mut_slice()[..value.len()].copy_from_slice(value);
        buf
    }

    pub const fn capacity(&self) -> usize {
        match &self.buf {
            Buf::Stack(_) => N,
            Buf::Heap(heap) => heap.capacity(),
        }
    }

    pub const fn as_slice(&self) -> &[T] {
        match &self.buf {
            Buf::Stack(stack) => stack,
            Buf::Heap(heap) => heap.as_slice(),
        }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        match &mut self.buf {
            Buf::Stack(stack) => stack,
            Buf::Heap(heap) => heap.as_mut_slice(),
        }
    }

    pub const unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where I: [const] std::slice::SliceIndex<[T]> {
        unsafe { self.as_slice().get_unchecked(index) }
    }
    pub const unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where I: [const] std::slice::SliceIndex<[T]> {
        unsafe { self.as_mut_slice().get_unchecked_mut(index) }
    }

    pub fn into_vec(self, len: usize) -> Vec<T>
    where T: Clone {
        match self.buf {
            Buf::Stack(stack) => stack[..len].to_vec(),
            Buf::Heap(mut heap) => {
                unsafe { heap.set_len(len) };
                heap
            },
        }
    }
}

impl<const N: usize> StackBuf<Utf8Letter, N> {
    pub const fn as_str(&self) -> &str {
        unsafe {
            let letters = self.as_slice();
            let slice = std::slice::from_raw_parts(letters.as_ptr().cast(), letters.len() * 2);
            str::from_utf8_unchecked(slice)
        }
    }

    pub fn into_string(self, len: usize) -> String {
        let v = self.into_vec(len);
        unsafe {
            let (ptr, len, cap) = v.into_raw_parts();
            let vec = Vec::<u8>::from_raw_parts(ptr.cast(), len * 2, cap * 2);
            String::from_utf8_unchecked(vec)
        }
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for StackBuf<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}
