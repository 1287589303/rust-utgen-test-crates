fn index(&self, index: Span) -> &str {
        &self[index.range()]
    }