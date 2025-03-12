pub fn look(look: Look) -> Hir {
        let props = Properties::look(look);
        Hir { kind: HirKind::Look(look), props }
    }