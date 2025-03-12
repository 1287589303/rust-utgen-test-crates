pub fn iter(&self) -> ByteClassIter<'_> {
        ByteClassIter { classes: self, i: 0 }
    }