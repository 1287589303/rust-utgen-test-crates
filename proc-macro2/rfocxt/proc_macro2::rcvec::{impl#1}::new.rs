use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
pub(crate) struct RcVecBuilder<T> {
    inner: Vec<T>,
}
impl<T> RcVecBuilder<T> {
    pub(crate) fn new() -> Self {
        RcVecBuilder { inner: Vec::new() }
    }
    pub(crate) fn with_capacity(cap: usize) -> Self {
        RcVecBuilder {
            inner: Vec::with_capacity(cap),
        }
    }
    pub(crate) fn push(&mut self, element: T) {}
    pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {}
    pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {}
    pub(crate) fn build(self) -> RcVec<T> {}
}
