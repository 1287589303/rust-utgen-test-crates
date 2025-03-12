pub fn create_captures(&self) -> Captures {
        Captures::all(self.nfa.group_info().clone())
    }