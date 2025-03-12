pub fn iter(&self) -> ClassUnicodeIter<'_> {
        ClassUnicodeIter(self.set.iter())
    }