use std::intrinsics::const_eval_select;
extern crate memchr;

// FIXME(const-hack): Remove this and replace calls when `haystack.find(needle)` is introduced and constified.
pub(crate) const fn slice_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    const_eval_select((haystack, needle), slice_find_const, memchr::memmem::find)
}
const fn slice_find_const(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while idx <= haystack.len() - needle.len() {
        let window = haystack.get(idx..(idx + needle.len())).unwrap();
        if window == needle {
            return Some(idx);
        }
        idx += 1;
    }
    None
}
