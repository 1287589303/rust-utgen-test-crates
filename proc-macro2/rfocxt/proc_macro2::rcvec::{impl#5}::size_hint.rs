use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
#[derive(Clone)]
pub(crate) struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
#[derive(Clone)]
pub struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: ProcMacroAutoTraits,
}
impl<T> Iterator for RcVecIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
