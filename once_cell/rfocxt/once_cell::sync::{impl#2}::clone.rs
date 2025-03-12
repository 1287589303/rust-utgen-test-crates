use core::{
    cell::Cell, fmt, mem, ops::{Deref, DerefMut},
    panic::RefUnwindSafe,
};
use super::imp::OnceCell as Imp;
pub struct OnceCell<T>(Imp<T>);
impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> OnceCell<T> {
        match self.get() {
            Some(value) => Self::with_value(value.clone()),
            None => Self::new(),
        }
    }
    fn clone_from(&mut self, source: &Self) {}
}
impl<T> OnceCell<T> {
    pub const fn new() -> OnceCell<T> {
        OnceCell(Imp::new())
    }
    pub const fn with_value(value: T) -> OnceCell<T> {
        OnceCell(Imp::with_value(value))
    }
    pub fn get(&self) -> Option<&T> {
        if self.0.is_initialized() {
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }
    #[cfg(feature = "std")]
    pub fn wait(&self) -> &T {}
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {}
    #[inline]
    pub unsafe fn get_unchecked(&self) -> &T {}
    pub fn set(&self, value: T) -> Result<(), T> {}
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {}
    pub fn take(&mut self) -> Option<T> {}
    #[inline]
    pub fn into_inner(self) -> Option<T> {}
}
