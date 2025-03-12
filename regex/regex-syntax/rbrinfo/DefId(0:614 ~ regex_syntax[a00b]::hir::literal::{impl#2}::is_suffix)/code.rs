pub fn is_suffix(&self) -> bool {
        matches!(*self, ExtractKind::Suffix)
    }