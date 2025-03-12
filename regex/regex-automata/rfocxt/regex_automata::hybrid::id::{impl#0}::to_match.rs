#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct LazyStateID(u32);
impl LazyStateID {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    const MAX_BIT: usize = 31;
    #[cfg(target_pointer_width = "16")]
    const MAX_BIT: usize = 15;
    const MASK_UNKNOWN: usize = 1 << (LazyStateID::MAX_BIT);
    const MASK_DEAD: usize = 1 << (LazyStateID::MAX_BIT - 1);
    const MASK_QUIT: usize = 1 << (LazyStateID::MAX_BIT - 2);
    const MASK_START: usize = 1 << (LazyStateID::MAX_BIT - 3);
    const MASK_MATCH: usize = 1 << (LazyStateID::MAX_BIT - 4);
    const MAX: usize = LazyStateID::MASK_MATCH - 1;
    #[inline]
    pub(crate) fn new(id: usize) -> Result<LazyStateID, LazyStateIDError> {}
    #[inline]
    const fn new_unchecked(id: usize) -> LazyStateID {
        LazyStateID(id as u32)
    }
    #[inline]
    pub(crate) fn as_usize_untagged(&self) -> usize {}
    #[inline]
    pub(crate) const fn as_usize_unchecked(&self) -> usize {
        self.0 as usize
    }
    #[inline]
    pub(crate) const fn to_unknown(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_dead(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_quit(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_start(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_match(&self) -> LazyStateID {
        LazyStateID::new_unchecked(self.as_usize_unchecked() | LazyStateID::MASK_MATCH)
    }
    #[inline]
    pub const fn is_tagged(&self) -> bool {}
    #[inline]
    pub const fn is_unknown(&self) -> bool {}
    #[inline]
    pub const fn is_dead(&self) -> bool {}
    #[inline]
    pub const fn is_quit(&self) -> bool {}
    #[inline]
    pub const fn is_start(&self) -> bool {}
    #[inline]
    pub const fn is_match(&self) -> bool {}
}
