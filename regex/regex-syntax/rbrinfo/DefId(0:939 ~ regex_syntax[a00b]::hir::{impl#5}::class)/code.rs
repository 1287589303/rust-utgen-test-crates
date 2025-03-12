pub fn class(class: Class) -> Hir {
        if class.is_empty() {
            return Hir::fail();
        } else if let Some(bytes) = class.literal() {
            return Hir::literal(bytes);
        }
        let props = Properties::class(&class);
        Hir { kind: HirKind::Class(class), props }
    }