fn try_clear_cache(&mut self) -> Result<(), CacheError> {
        let c = self.dfa.get_config();
        if let Some(min_count) = c.get_minimum_cache_clear_count() {
            if self.cache.clear_count >= min_count {
                if let Some(min_bytes_per) = c.get_minimum_bytes_per_state() {
                    let len = self.cache.search_total_len();
                    let min_bytes =
                        min_bytes_per.saturating_mul(self.cache.states.len());
                    // If we've searched 0 bytes then probably something has
                    // gone wrong and the lazy DFA search implementation isn't
                    // correctly updating the search progress state.
                    if len == 0 {
                        trace!(
                            "number of bytes searched is 0, but \
                             a minimum bytes per state searched ({}) is \
                             enabled, maybe Cache::search_update \
                             is not being used?",
                            min_bytes_per,
                        );
                    }
                    if len < min_bytes {
                        trace!(
                            "lazy DFA cache has been cleared {} times, \
                             which exceeds the limit of {}, \
                             AND its bytes searched per state is less \
                             than the configured minimum of {}, \
                             therefore lazy DFA is giving up \
                             (bytes searched since cache clear = {}, \
                              number of states = {})",
                            self.cache.clear_count,
                            min_count,
                            min_bytes_per,
                            len,
                            self.cache.states.len(),
                        );
                        return Err(CacheError::bad_efficiency());
                    } else {
                        trace!(
                            "lazy DFA cache has been cleared {} times, \
                             which exceeds the limit of {}, \
                             AND its bytes searched per state is greater \
                             than the configured minimum of {}, \
                             therefore lazy DFA is continuing! \
                             (bytes searched since cache clear = {}, \
                              number of states = {})",
                            self.cache.clear_count,
                            min_count,
                            min_bytes_per,
                            len,
                            self.cache.states.len(),
                        );
                    }
                } else {
                    trace!(
                        "lazy DFA cache has been cleared {} times, \
                         which exceeds the limit of {}, \
                         since there is no configured bytes per state \
                         minimum, lazy DFA is giving up",
                        self.cache.clear_count,
                        min_count,
                    );
                    return Err(CacheError::too_many_cache_clears());
                }
            }
        }
        self.clear_cache();
        Ok(())
    }