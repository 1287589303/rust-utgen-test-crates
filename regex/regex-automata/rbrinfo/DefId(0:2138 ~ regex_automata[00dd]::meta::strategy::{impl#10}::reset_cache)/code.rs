fn reset_cache(&self, cache: &mut Cache) {
        self.core.reset_cache(cache);
        cache.revhybrid.reset(&self.hybrid);
    }