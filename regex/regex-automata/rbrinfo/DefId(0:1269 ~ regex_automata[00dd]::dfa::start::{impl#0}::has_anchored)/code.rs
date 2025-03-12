pub(crate) fn has_anchored(&self) -> bool {
        matches!(*self, StartKind::Both | StartKind::Anchored)
    }