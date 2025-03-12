pub fn is_prefix(&self) -> bool {
        matches!(*self, ExtractKind::Prefix)
    }