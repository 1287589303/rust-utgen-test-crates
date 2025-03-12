use core::num::NonZeroUsize;
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
}
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>;
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
impl U32 for u32 {
    fn as_usize(self) -> usize {
        self as usize
    }
}
