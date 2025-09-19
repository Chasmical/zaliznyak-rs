use crate::word::{Utf8Letter, Utf8LetterSlice};
use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

//   Layout for [T; N] <= 2 usizes
// |--|--|--|--|--|--|--|--|--|--|--|--|
// |00 00 00 00| <-- Stack Buffer  --> |
// | Unique<T> |  Length   | Capacity  |
//
//   Layout for [T; N] > 2 usizes
// |--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
// |00| <------------- Stack Buffer -------------> |
// |01|--------| Unique<T> |  Length   | Capacity  |

pub(crate) enum StackBuf<T, const N: usize> {
    Stack([MaybeUninit<T>; N]),
    Heap(Vec<T>),
}

impl<T, const N: usize> StackBuf<T, N> {
    pub fn with_capacity(cap: usize) -> Self
    where T: Copy {
        #[allow(clippy::uninit_vec)]
        if cap <= N {
            Self::Stack([MaybeUninit::uninit(); N])
        } else {
            let mut vec = Vec::with_capacity(cap);
            unsafe { vec.set_len(cap) };
            Self::Heap(vec)
        }
    }

    pub const fn capacity(&self) -> usize {
        match self {
            Self::Stack(_) => N,
            Self::Heap(heap) => heap.capacity(),
        }
    }
    pub const fn as_slice(&self) -> &[T] {
        match self {
            Self::Stack(stack) => unsafe { stack.assume_init_ref() },
            Self::Heap(heap) => heap.as_slice(),
        }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            Self::Stack(stack) => unsafe { stack.assume_init_mut() },
            Self::Heap(heap) => heap.as_mut_slice(),
        }
    }

    pub fn into_vec(self, len: usize) -> Vec<T>
    where T: Clone {
        debug_assert!(len <= self.capacity());

        match self {
            Self::Stack(stack) => unsafe { stack[..len].assume_init_ref() }.to_vec(),
            Self::Heap(mut heap) => {
                unsafe { heap.set_len(len) };
                heap
            },
        }
    }
}

impl<const N: usize> StackBuf<Utf8Letter, N> {
    pub const fn as_str(&self) -> &str {
        self.as_slice().as_str()
    }
    pub fn into_string(self, len: usize) -> String {
        let v = self.into_vec(len);
        let (ptr, len, cap) = v.into_raw_parts();
        let vec = unsafe { Vec::<u8>::from_raw_parts(ptr.cast(), len * 2, cap * 2) };
        unsafe { String::from_utf8_unchecked(vec) }
    }
}

impl<T: Copy, const N: usize> const Default for StackBuf<T, N> {
    fn default() -> Self {
        Self::Stack([MaybeUninit::uninit(); N])
    }
}

impl<T: Copy, const N: usize> From<&[T]> for StackBuf<T, N> {
    fn from(value: &[T]) -> Self {
        let mut buf = Self::with_capacity(value.len());
        buf.as_mut_slice()[..value.len()].copy_from_slice(value);
        buf
    }
}
impl<T: Copy, const N: usize, const K: usize> From<[T; K]> for StackBuf<T, N> {
    fn from(value: [T; K]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T, const N: usize> const Deref for StackBuf<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T, const N: usize> const DerefMut for StackBuf<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
impl<T, const N: usize> const AsRef<[T]> for StackBuf<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T, const N: usize> const AsMut<[T]> for StackBuf<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for StackBuf<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}
