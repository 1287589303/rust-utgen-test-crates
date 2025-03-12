pub fn set_size_limit(
        &mut self,
        limit: Option<usize>,
    ) -> Result<(), BuildError> {
        self.size_limit = limit;
        self.check_size_limit()
    }