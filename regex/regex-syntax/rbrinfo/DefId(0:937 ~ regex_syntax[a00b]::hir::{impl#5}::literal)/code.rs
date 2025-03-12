pub fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir {
        let bytes = lit.into();
        if bytes.is_empty() {
            return Hir::empty();
        }

        let lit = Literal(bytes);
        let props = Properties::literal(&lit);
        Hir { kind: HirKind::Literal(lit), props }
    }