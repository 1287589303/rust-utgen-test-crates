pub(crate) fn iter(&self) -> ByteSetIter {
        ByteSetIter { set: self, b: 0 }
    }