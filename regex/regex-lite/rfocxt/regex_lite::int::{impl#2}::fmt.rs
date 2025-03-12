use core::num::NonZeroUsize;
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
impl core::fmt::Debug for NonMaxUsize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}
impl NonMaxUsize {
    pub(crate) fn new(value: usize) -> Option<NonMaxUsize> {}
    pub(crate) fn get(self) -> usize {
        self.0.get().wrapping_sub(1)
    }
}
