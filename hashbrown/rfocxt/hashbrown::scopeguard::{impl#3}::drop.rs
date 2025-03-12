use core::{
    mem::ManuallyDrop, ops::{Deref, DerefMut},
    ptr,
};
pub struct ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    dropfn: F,
    value: T,
}
impl<T, F> Drop for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    #[inline]
    fn drop(&mut self) {
        (self.dropfn)(&mut self.value);
    }
}
