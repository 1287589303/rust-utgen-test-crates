fn empty() -> Hir {
        let kind = HirKind::Empty;
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: Some(0),
        }
    }