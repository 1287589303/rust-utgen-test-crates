pub fn build(&self) -> Result<Regex, Error> {
        let hir = Hir::parse(self.hir_config, &self.pattern)?;
        let nfa = NFA::new(self.nfa_config, self.pattern.clone(), &hir)?;
        let pikevm = Arc::new(PikeVM::new(nfa));
        let pool = {
            let pikevm = Arc::clone(&pikevm);
            let create = Box::new(move || Cache::new(&pikevm));
            CachePool::new(create)
        };
        Ok(Regex { pikevm, pool })
    }