use crate::{hybrid::id::LazyStateIDError, nfa, util::search::Anchored};
#[derive(Clone, Debug)]
pub struct CacheError(());
impl core::fmt::Display for CacheError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "lazy DFA cache has been cleared too many times")
    }
}
