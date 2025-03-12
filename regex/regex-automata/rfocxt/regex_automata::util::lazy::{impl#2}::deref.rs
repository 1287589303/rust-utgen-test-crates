use core::fmt;
pub struct Lazy<T, F = fn() -> T>(lazy::Lazy<T, F>);
pub(super) struct Lazy<T, F> {
    state: AtomicU8,
    create: Cell<Option<F>>,
    data: Cell<MaybeUninit<T>>,
}
#[derive(Debug)]
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}
impl<T, F: Fn() -> T> core::ops::Deref for Lazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        Lazy::get(self)
    }
}
impl<T, F: Fn() -> T> Lazy<T, F> {
    pub fn get(this: &Lazy<T, F>) -> &T {
        this.0.get()
    }
}
