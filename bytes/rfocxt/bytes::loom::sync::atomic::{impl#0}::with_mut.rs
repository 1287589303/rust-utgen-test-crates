#[cfg(not(feature = "extra-platforms"))]
pub(crate) use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
pub(crate) trait AtomicMut<T> {}
impl<T> AtomicMut<T> for AtomicPtr<T> {
    fn with_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut *mut T) -> R,
    {
        f(self.get_mut())
    }
}
