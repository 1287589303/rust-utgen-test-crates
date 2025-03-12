fn look(look: Look) -> Hir {
        let kind = HirKind::Look(look);
        Hir {
            kind,
            is_start_anchored: matches!(look, Look::Start),
            is_match_empty: true,
            static_explicit_captures_len: Some(0),
        }
    }