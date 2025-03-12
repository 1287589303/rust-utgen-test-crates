fn index_mut(&mut self, index: Span) -> &mut [u8] {
        &mut self[index.range()]
    }