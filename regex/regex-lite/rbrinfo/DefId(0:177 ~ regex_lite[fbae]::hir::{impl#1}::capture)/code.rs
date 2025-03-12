fn capture(cap: Capture) -> Hir {
        let is_start_anchored = cap.sub.is_start_anchored;
        let is_match_empty = cap.sub.is_match_empty;
        let static_explicit_captures_len = cap
            .sub
            .static_explicit_captures_len
            .map(|len| len.saturating_add(1));
        let kind = HirKind::Capture(cap);
        Hir {
            kind,
            is_start_anchored,
            is_match_empty,
            static_explicit_captures_len,
        }
    }