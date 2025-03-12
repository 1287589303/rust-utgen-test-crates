fn check_size_limit(&self) -> Result<(), BuildError> {
        if let Some(limit) = self.size_limit {
            if self.memory_usage() > limit {
                return Err(BuildError::exceeded_size_limit(limit));
            }
        }
        Ok(())
    }