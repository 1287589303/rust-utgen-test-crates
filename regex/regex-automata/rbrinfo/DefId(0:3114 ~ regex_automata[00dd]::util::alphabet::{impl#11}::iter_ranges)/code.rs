pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {
        ByteSetRangeIter { set: self, b: 0 }
    }