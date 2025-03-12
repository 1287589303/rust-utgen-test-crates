use core::{fmt, mem};
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
impl Tag {
    pub(crate) const EMPTY: Tag = Tag(0b1111_1111);
    pub(crate) const DELETED: Tag = Tag(0b1000_0000);
    #[inline]
    pub(crate) const fn is_full(self) -> bool {}
    #[inline]
    pub(crate) const fn is_special(self) -> bool {}
    #[inline]
    pub(crate) const fn special_is_empty(self) -> bool {}
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn full(hash: u64) -> Tag {
        const MIN_HASH_LEN: usize = if mem::size_of::<usize>() < mem::size_of::<u64>() {
            mem::size_of::<usize>()
        } else {
            mem::size_of::<u64>()
        };
        let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
        Tag((top7 & 0x7f) as u8)
    }
}
