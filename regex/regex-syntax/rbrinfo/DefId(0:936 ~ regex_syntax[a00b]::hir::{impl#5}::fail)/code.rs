pub fn fail() -> Hir {
        let class = Class::Bytes(ClassBytes::empty());
        let props = Properties::class(&class);
        // We can't just call Hir::class here because it defers to Hir::fail
        // in order to canonicalize the Hir value used to represent "cannot
        // match."
        Hir { kind: HirKind::Class(class), props }
    }