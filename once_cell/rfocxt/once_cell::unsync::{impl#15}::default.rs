use core::{
    cell::{Cell, UnsafeCell},
    fmt, mem, ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
pub struct OnceCell<T>(Imp<T>);
#[derive(Debug)]
pub(crate) struct OnceCell<T> {
    queue: AtomicPtr<Waiter>,
    value: UnsafeCell<Option<T>>,
}
impl<T: Default> Default for Lazy<T> {
    fn default() -> Lazy<T> {
        Lazy::new(T::default)
    }
}
impl<T, F> Lazy<T, F> {
    pub const fn new(init: F) -> Lazy<T, F> {
        Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(init)),
        }
    }
    pub fn into_value(this: Lazy<T, F>) -> Result<T, F> {}
}
