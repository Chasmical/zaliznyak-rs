// FIXME(const-hack): Remove this and replace calls when `haystack.find(needle)` is introduced and constified.
pub(crate) const fn slice_find<T: [const] PartialEq>(
    haystack: &[T],
    needle: &[T],
) -> Option<usize> {
    let mut idx = 0;
    while idx <= haystack.len() - needle.len() {
        let window = unsafe { haystack.get_unchecked(idx..(idx + needle.len())) };
        if window == needle {
            return Some(idx);
        }
        idx += 1;
    }
    None
}
