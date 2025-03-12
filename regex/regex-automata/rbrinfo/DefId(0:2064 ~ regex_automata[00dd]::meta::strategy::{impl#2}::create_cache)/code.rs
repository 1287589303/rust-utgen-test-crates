fn create_cache(&self) -> Cache {
        Cache {
            capmatches: Captures::all(self.group_info().clone()),
            pikevm: wrappers::PikeVMCache::none(),
            backtrack: wrappers::BoundedBacktrackerCache::none(),
            onepass: wrappers::OnePassCache::none(),
            hybrid: wrappers::HybridCache::none(),
            revhybrid: wrappers::ReverseHybridCache::none(),
        }
    }