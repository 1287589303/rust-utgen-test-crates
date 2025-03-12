fn from(index: u8) -> SmallIndex {
        SmallIndex::new_unchecked(usize::from(index))
    }