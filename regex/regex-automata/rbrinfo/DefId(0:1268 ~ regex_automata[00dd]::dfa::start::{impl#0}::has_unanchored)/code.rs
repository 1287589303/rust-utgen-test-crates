pub(crate) fn has_unanchored(&self) -> bool {
        matches!(*self, StartKind::Both | StartKind::Unanchored)
    }