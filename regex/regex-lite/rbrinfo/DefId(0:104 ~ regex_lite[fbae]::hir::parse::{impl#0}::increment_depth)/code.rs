fn increment_depth(&self) -> Result<u32, Error> {
        let old = self.depth.get();
        if old > self.config.nest_limit {
            return Err(Error::new(ERR_TOO_MUCH_NESTING));
        }
        // OK because our depth starts at 0, and we return an error if it
        // ever reaches the limit. So the call depth can never exceed u32::MAX.
        let new = old.checked_add(1).unwrap();
        self.depth.set(new);
        Ok(old)
    }