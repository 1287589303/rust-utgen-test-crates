pub fn iter(&self) -> ClassBytesIter<'_> {
        ClassBytesIter(self.set.iter())
    }