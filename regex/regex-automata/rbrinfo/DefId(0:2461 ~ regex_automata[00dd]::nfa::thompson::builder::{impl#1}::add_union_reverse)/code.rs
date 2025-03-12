pub fn add_union_reverse(
        &mut self,
        alternates: Vec<StateID>,
    ) -> Result<StateID, BuildError> {
        self.add(State::UnionReverse { alternates })
    }