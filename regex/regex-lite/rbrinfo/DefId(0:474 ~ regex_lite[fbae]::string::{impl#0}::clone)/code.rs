fn clone(&self) -> Regex {
        let pikevm = Arc::clone(&self.pikevm);
        let pool = {
            let pikevm = Arc::clone(&self.pikevm);
            let create = Box::new(move || Cache::new(&pikevm));
            CachePool::new(create)
        };
        Regex { pikevm, pool }
    }