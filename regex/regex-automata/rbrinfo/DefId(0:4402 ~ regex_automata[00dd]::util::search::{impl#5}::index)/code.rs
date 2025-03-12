fn index(&self, index: Span) -> &[u8] {
        &self[index.range()]
    }