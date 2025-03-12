fn iter(&self) -> StartStateIter<'_> {
        StartStateIter { st: self.as_ref(), i: 0 }
    }