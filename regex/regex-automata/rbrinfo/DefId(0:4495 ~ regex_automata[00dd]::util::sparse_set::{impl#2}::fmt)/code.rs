fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let elements: Vec<StateID> = self.iter().collect();
        f.debug_tuple("SparseSet").field(&elements).finish()
    }