fn create_cache(&self) -> Cache {
        Cache {
            capmatches: Captures::all(self.group_info().clone()),
            pikevm: self.pikevm.create_cache(),
            backtrack: self.backtrack.create_cache(),
            onepass: self.onepass.create_cache(),
            hybrid: self.hybrid.create_cache(),
            revhybrid: wrappers::ReverseHybridCache::none(),
        }
    }