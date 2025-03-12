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
impl<T, F> Lazy<T, F> {
    pub(super) const fn new(create: F) -> Lazy<T, F> {
        Lazy {
            state: AtomicU8::new(LAZY_STATE_INIT),
            create: Cell::new(Some(create)),
            data: Cell::new(MaybeUninit::uninit()),
        }
    }
}
