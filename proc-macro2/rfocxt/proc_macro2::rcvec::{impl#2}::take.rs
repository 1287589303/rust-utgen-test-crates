use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
pub(crate) struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
}
pub(crate) struct RcVecBuilder<T> {
    inner: Vec<T>,
}
impl<'a, T> RcVecMut<'a, T> {
    pub(crate) fn push(&mut self, element: T) {}
    pub(crate) fn extend(&mut self, iter: impl IntoIterator<Item = T>) {}
    pub(crate) fn as_mut(&mut self) -> RcVecMut<T> {}
    pub(crate) fn take(self) -> RcVecBuilder<T> {
        let vec = mem::take(self.inner);
        RcVecBuilder { inner: vec }
    }
}
