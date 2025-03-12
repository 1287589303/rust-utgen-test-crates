fn into_parts(mut self) -> (HirKind, Properties) {
        (
            core::mem::replace(&mut self.kind, HirKind::Empty),
            core::mem::replace(&mut self.props, Properties::empty()),
        )
    }