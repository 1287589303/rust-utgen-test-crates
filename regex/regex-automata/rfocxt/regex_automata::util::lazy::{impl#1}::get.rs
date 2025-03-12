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
impl<T, F: Fn() -> T> Lazy<T, F> {
    pub fn get(this: &Lazy<T, F>) -> &T {
        this.0.get()
    }
}
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub(super) fn get(&self) -> &T {
        while self.state.load(Ordering::Acquire) != LAZY_STATE_DONE {
            let result = self
                .state
                .compare_exchange(
                    LAZY_STATE_INIT,
                    LAZY_STATE_BUSY,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            if let Ok(_) = result {
                let create = unsafe {
                    (*self.create.as_ptr())
                        .take()
                        .expect(
                            "Lazy's create function panicked, \
                             preventing initialization,
                             poisoning current thread",
                        )
                };
                let guard = Guard { state: &self.state };
                unsafe {
                    (*self.data.as_ptr()).as_mut_ptr().write(create());
                }
                core::mem::forget(guard);
                self.state.store(LAZY_STATE_DONE, Ordering::Release);
                break;
            }
            core::hint::spin_loop();
        }
        self.poll().unwrap()
    }
    fn poll(&self) -> Option<&T> {}
}
