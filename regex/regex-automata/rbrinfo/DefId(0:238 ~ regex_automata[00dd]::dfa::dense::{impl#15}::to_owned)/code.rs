fn to_owned(&self) -> TransitionTable<alloc::vec::Vec<u32>> {
        TransitionTable {
            table: self.table.as_ref().to_vec(),
            classes: self.classes.clone(),
            stride2: self.stride2,
        }
    }