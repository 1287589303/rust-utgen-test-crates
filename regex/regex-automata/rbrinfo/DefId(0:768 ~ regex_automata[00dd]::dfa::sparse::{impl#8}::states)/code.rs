fn states(&self) -> StateIter<'_, T> {
        StateIter { trans: self, id: DEAD.as_usize() }
    }