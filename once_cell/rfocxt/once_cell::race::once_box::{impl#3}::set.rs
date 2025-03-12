use super::atomic::{AtomicPtr, Ordering};
use core::{marker::PhantomData, ptr};
use alloc::boxed::Box;
pub struct OnceBox<T> {
    inner: AtomicPtr<T>,
    ghost: PhantomData<Option<Box<T>>>,
}
impl<T> OnceBox<T> {
    pub const fn new() -> OnceBox<T> {}
    pub fn with_value(value: Box<T>) -> Self {
        OnceBox {
            inner: AtomicPtr::new(Box::into_raw(value)),
            ghost: PhantomData,
        }
    }
    pub fn get(&self) -> Option<&T> {}
    pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {
        let ptr = Box::into_raw(value);
        let exchange = self
            .inner
            .compare_exchange(ptr::null_mut(), ptr, Ordering::AcqRel, Ordering::Acquire);
        if exchange.is_err() {
            let value = unsafe { Box::from_raw(ptr) };
            return Err(value);
        }
        Ok(())
    }
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> Box<T>,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<Box<T>, E>,
    {}
}
