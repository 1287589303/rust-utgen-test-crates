pub fn get_group_by_name(&self, name: &str) -> Option<Span> {
        let index = self.group_info().to_index(self.pattern()?, name)?;
        self.get_group(index)
    }