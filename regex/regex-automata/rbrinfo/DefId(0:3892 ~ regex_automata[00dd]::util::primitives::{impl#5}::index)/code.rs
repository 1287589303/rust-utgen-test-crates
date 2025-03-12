fn index(&self, index: SmallIndex) -> &T {
        &self[index.as_usize()]
    }