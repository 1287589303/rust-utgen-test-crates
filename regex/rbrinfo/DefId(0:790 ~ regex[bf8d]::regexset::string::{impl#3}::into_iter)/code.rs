fn into_iter(self) -> Self::IntoIter {
        let it = 0..self.0.capacity();
        SetMatchesIntoIter { patset: self.0, it }
    }