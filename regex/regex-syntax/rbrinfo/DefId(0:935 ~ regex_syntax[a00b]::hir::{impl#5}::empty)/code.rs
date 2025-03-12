pub fn empty() -> Hir {
        let props = Properties::empty();
        Hir { kind: HirKind::Empty, props }
    }