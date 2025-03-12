fn char(ch: char) -> Hir {
        let kind = HirKind::Char(ch);
        Hir {
            kind,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(0),
        }
    }