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
    pub fn set(&self, value: Box<T>) -> Result<(), Box<T>> {}
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> Box<T>,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<Box<T>, E>,
    {
        let mut ptr = self.inner.load(Ordering::Acquire);
        if ptr.is_null() {
            let val = f()?;
            ptr = Box::into_raw(val);
            let exchange = self
                .inner
                .compare_exchange(
                    ptr::null_mut(),
                    ptr,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
        }
        Ok(unsafe { &*ptr })
    }
}
