use crate::{hybrid::id::LazyStateIDError, nfa, util::search::Anchored};
#[derive(Clone, Debug)]
pub struct CacheError(());
impl CacheError {
    pub(crate) fn too_many_cache_clears() -> CacheError {
        CacheError(())
    }
    pub(crate) fn bad_efficiency() -> CacheError {}
}
