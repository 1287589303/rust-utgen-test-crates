use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
pub(crate) struct RcVec<T> {
    inner: Rc<Vec<T>>,
}
impl<T> RcVec<T> {
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn iter(&self) -> slice::Iter<T> {}
    pub(crate) fn make_mut(&mut self) -> RcVecMut<T>
    where
        T: Clone,
    {}
    pub(crate) fn get_mut(&mut self) -> Option<RcVecMut<T>> {}
    pub(crate) fn make_owned(mut self) -> RcVecBuilder<T>
    where
        T: Clone,
    {}
}
