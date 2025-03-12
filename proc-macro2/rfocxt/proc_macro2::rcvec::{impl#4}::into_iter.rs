use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
pub(crate) struct RcVecBuilder<T> {
    inner: Vec<T>,
}
#[derive(Clone)]
pub(crate) struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
impl<T> IntoIterator for RcVecBuilder<T> {
    type Item = T;
    type IntoIter = RcVecIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        RcVecIntoIter {
            inner: self.inner.into_iter(),
        }
    }
}
