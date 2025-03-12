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
    pub(crate) const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }
    #[inline]
    pub(crate) const fn special_is_empty(self) -> bool {}
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn full(hash: u64) -> Tag {}
}
