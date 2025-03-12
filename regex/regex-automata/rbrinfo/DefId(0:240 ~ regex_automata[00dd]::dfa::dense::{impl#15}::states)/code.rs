fn states(&self) -> StateIter<'_, T> {
        StateIter {
            tt: self,
            it: self.table().chunks(self.stride()).enumerate(),
        }
    }