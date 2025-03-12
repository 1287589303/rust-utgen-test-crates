pub fn create_captures(&self) -> Captures {
        Captures::all(self.get_nfa().group_info().clone())
    }