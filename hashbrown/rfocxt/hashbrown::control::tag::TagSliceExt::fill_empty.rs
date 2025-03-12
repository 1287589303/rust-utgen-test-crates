use core::{fmt, mem};
pub(crate) trait TagSliceExt {
    fn fill_tag(&mut self, tag: Tag);
    #[inline]
    fn fill_empty(&mut self) {
        self.fill_tag(Tag::EMPTY)
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
