fn index_mut(&mut self, index: SmallIndex) -> &mut T {
        &mut self[index.as_usize()]
    }