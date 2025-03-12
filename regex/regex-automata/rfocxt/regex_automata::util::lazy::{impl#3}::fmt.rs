use core::fmt;
pub struct Lazy<T, F = fn() -> T>(lazy::Lazy<T, F>);
#[derive(Debug)]
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}
pub(super) struct Lazy<T, F> {
    state: AtomicU8,
    create: Cell<Option<F>>,
    data: Cell<MaybeUninit<T>>,
}
impl<T: fmt::Debug, F: Fn() -> T> fmt::Debug for Lazy<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
