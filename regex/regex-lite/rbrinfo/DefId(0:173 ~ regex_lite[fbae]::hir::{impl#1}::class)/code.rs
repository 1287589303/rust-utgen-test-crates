fn class(class: Class) -> Hir {
        let kind = HirKind::Class(class);
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(0),
        }
    }