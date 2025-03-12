fn fail() -> Hir {
        let kind = HirKind::Class(Class { ranges: vec![] });
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(0),
        }
    }