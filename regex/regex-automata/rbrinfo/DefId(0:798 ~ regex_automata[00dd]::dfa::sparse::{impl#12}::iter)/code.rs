fn iter(&self) -> StartStateIter<'_, T> {
        StartStateIter { st: self, i: 0 }
    }