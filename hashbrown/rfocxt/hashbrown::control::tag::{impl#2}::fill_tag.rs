use core::{fmt, mem};
pub(crate) trait TagSliceExt {
    fn fill_tag(&mut self, tag: Tag);
    #[inline]
    fn fill_empty(&mut self) {
        self.fill_tag(Tag::EMPTY)
    }
}
#[cfg(not(feature = "equivalent"))]
pub trait Equivalent<K: ?Sized> {
    fn equivalent(&self, key: &K) -> bool;
}
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
impl TagSliceExt for [Tag] {
    #[inline]
    fn fill_tag(&mut self, tag: Tag) {
        unsafe { self.as_mut_ptr().write_bytes(tag.0, self.len()) }
    }
}
