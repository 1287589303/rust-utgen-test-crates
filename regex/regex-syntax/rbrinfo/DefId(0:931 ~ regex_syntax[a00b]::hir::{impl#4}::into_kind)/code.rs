pub fn into_kind(mut self) -> HirKind {
        core::mem::replace(&mut self.kind, HirKind::Empty)
    }