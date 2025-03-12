pub fn create_captures(&self) -> Captures {
        Captures::all(self.group_info().clone())
    }