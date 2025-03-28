pub struct PoolGuard<'a, T: Send, F: Fn() -> T>(inner::PoolGuard<'a, T, F>);
pub(super) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
impl<'a, T: Send, F: Fn() -> T> core::ops::Deref for PoolGuard<'a, T, F> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        self.0.value()
    }
}
impl<'a, T: Send, F: Fn() -> T> PoolGuard<'a, T, F> {
    #[inline]
    pub(super) fn value(&self) -> &T {
        self.value.as_deref().unwrap()
    }
    #[inline]
    pub(super) fn value_mut(&mut self) -> &mut T {}
    #[inline]
    pub(super) fn put(this: PoolGuard<'_, T, F>) {}
    #[inline(always)]
    fn put_imp(&mut self) {}
}
