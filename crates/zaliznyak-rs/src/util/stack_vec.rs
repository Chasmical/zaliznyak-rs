use crate::word::Utf8Letter;
use std::{
    hash::Hash,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub(crate) struct StackVec<T, const N: usize> {
    buf: Buf<T, N>,
    len: usize,
}

//   Buf's Layout for [T; N] <= 1 usize
// |--|--|--|--|--|--|--|--|
// |00 00 00 00| <-Stack-> |
// | NonNull<T>| Capacity  |
//
//   Buf's Layout for [T; N] > 1 usize
// |--|--|--|--|--|--|--|--|--|--|--|--|
// |00| <------- Stack Buffer -------> |
// |01|--------| NonNull<T>| Capacity  |

enum Buf<T, const N: usize> {
    Stack([MaybeUninit<T>; N]),
    Heap(NonNull<T>, usize),
}

impl<T, const N: usize> StackVec<T, N> {
    pub fn with_capacity(cap: usize) -> Self {
        if cap <= N {
            Self { buf: Buf::Stack(MaybeUninit::uninit().transpose()), len: 0 }
        } else {
            let (ptr, len, cap) = Vec::with_capacity(cap).into_parts();
            Self { buf: Buf::Heap(ptr, cap), len }
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        if let Buf::Heap(_, cap) = &self.buf { *cap } else { N }
    }

    pub const unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity());
        self.len = len;
    }

    pub const fn as_ptr(&self) -> *const T {
        match &self.buf {
            Buf::Stack(stack) => stack.as_ptr().cast_init(),
            Buf::Heap(ptr, _) => ptr.as_ptr(),
        }
    }
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        match &mut self.buf {
            Buf::Stack(stack) => stack.as_mut_ptr().cast_init(),
            Buf::Heap(ptr, _) => ptr.as_ptr(),
        }
    }

    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len) }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }

    pub const fn slice_full_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        match &mut self.buf {
            Buf::Stack(stack) => stack,
            Buf::Heap(ptr, cap) => unsafe {
                std::slice::from_raw_parts_mut(ptr.as_ptr().cast_uninit(), *cap)
            },
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        let me = ManuallyDrop::new(self);

        match &me.buf {
            Buf::Stack(stack) => {
                let mut vec = Vec::with_capacity(me.len);
                unsafe {
                    let src = stack.as_ptr().cast_init();
                    std::ptr::copy_nonoverlapping(src, vec.as_mut_ptr(), me.len);
                    vec.set_len(me.len);
                }
                vec
            },
            Buf::Heap(ptr, cap) => unsafe { Vec::from_parts(*ptr, me.len, *cap) },
        }
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        if let Buf::Heap(ptr, cap) = self.buf {
            drop(unsafe { Vec::from_parts(ptr, self.len, cap) });
        }
    }
}

impl<T, const N: usize> const Default for StackVec<T, N> {
    fn default() -> Self {
        Self { buf: Buf::Stack(MaybeUninit::uninit().transpose()), len: 0 }
    }
}
impl<T: Clone, const N: usize> Clone for StackVec<T, N> {
    fn clone(&self) -> Self {
        Self::from(self.as_slice())
    }
}

impl<T: [const] PartialEq, const N: usize> const PartialEq for StackVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<T: Eq, const N: usize> Eq for StackVec<T, N> {}

impl<T: Hash, const N: usize> Hash for StackVec<T, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T: Clone, const N: usize> From<&[T]> for StackVec<T, N> {
    fn from(value: &[T]) -> Self {
        let mut vec = Self::with_capacity(value.len());
        vec.len = value.len();
        vec.as_mut_slice().clone_from_slice(value);
        vec
    }
}
impl<T: Copy, const N: usize, const K: usize> From<[T; K]> for StackVec<T, N> {
    fn from(value: [T; K]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T, const N: usize> const Deref for StackVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T, const N: usize> const DerefMut for StackVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
impl<T, const N: usize> const AsRef<[T]> for StackVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T, const N: usize> const AsMut<[T]> for StackVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for StackVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<const N: usize> StackVec<Utf8Letter, N> {
    pub fn into_string(self) -> String {
        // TODO: Is this kind of casting safe? GlobalAlloc::dealloc's docs say that the layout
        //   provided to alloc and dealloc MUST be the same, even if the alignment is less strict!
        unsafe {
            let (ptr, len, cap) = self.into_vec().into_raw_parts();
            let vec = Vec::<u8>::from_raw_parts(ptr.cast(), len * 2, cap * 2);
            String::from_utf8_unchecked(vec)
        }
    }
}
