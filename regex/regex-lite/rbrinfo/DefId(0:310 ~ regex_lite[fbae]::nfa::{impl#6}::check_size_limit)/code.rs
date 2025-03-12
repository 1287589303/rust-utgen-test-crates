fn check_size_limit(&self) -> Result<(), Error> {
        if let Some(limit) = self.config.size_limit {
            if self.nfa.borrow().memory_usage() > limit {
                return Err(Error::new("compiled regex exceeded size limit"));
            }
        }
        Ok(())
    }