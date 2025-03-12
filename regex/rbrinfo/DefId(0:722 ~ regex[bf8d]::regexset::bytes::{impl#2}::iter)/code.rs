pub fn iter(&self) -> SetMatchesIter<'_> {
        SetMatchesIter(self.0.iter())
    }