fn create_cache(&self) -> Cache {
        let mut cache = self.core.create_cache();
        cache.revhybrid = self.hybrid.create_cache();
        cache
    }