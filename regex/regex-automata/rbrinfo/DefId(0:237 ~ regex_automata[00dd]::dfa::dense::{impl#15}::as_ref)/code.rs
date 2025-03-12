fn as_ref(&self) -> TransitionTable<&'_ [u32]> {
        TransitionTable {
            table: self.table.as_ref(),
            classes: self.classes.clone(),
            stride2: self.stride2,
        }
    }