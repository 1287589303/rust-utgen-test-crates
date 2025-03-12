#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LazyStateIDError {
    attempted: u64,
}
impl LazyStateIDError {
    pub(crate) fn attempted(&self) -> u64 {
        self.attempted
    }
}
