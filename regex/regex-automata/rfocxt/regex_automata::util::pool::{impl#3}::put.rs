pub struct PoolGuard<'a, T: Send, F: Fn() -> T>(inner::PoolGuard<'a, T, F>);
pub(super) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
impl<'a, T: Send, F: Fn() -> T> PoolGuard<'a, T, F> {
    #[inline]
    pub(super) fn put(this: PoolGuard<'_, T, F>) {
        let mut this = core::mem::ManuallyDrop::new(this);
        this.put_imp();
    }
}
