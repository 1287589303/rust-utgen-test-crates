#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LazyStateIDError {
    attempted: u64,
}
impl core::fmt::Display for LazyStateIDError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "failed to create LazyStateID from {:?}, which exceeds {:?}", self
            .attempted(), LazyStateID::MAX,
        )
    }
}
impl LazyStateIDError {
    pub(crate) fn attempted(&self) -> u64 {
        self.attempted
    }
}
