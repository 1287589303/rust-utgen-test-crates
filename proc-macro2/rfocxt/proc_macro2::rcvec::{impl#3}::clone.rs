use alloc::rc::Rc;
use alloc::vec;
use core::mem;
use core::panic::RefUnwindSafe;
use core::slice;
pub(crate) struct RcVec<T> {
    inner: Rc<Vec<T>>,
}
impl<T> Clone for RcVec<T> {
    fn clone(&self) -> Self {
        RcVec {
            inner: Rc::clone(&self.inner),
        }
    }
}
