fn alternation(mut subs: Vec<Hir>) -> Hir {
        if subs.is_empty() {
            Hir::fail()
        } else if subs.len() == 1 {
            subs.pop().unwrap()
        } else {
            let mut it = subs.iter().peekable();
            let mut is_start_anchored =
                it.peek().map_or(false, |sub| sub.is_start_anchored);
            let mut is_match_empty =
                it.peek().map_or(false, |sub| sub.is_match_empty);
            let mut static_explicit_captures_len =
                it.peek().and_then(|sub| sub.static_explicit_captures_len);
            for sub in it {
                is_start_anchored = is_start_anchored && sub.is_start_anchored;
                is_match_empty = is_match_empty || sub.is_match_empty;
                if static_explicit_captures_len
                    != sub.static_explicit_captures_len
                {
                    static_explicit_captures_len = None;
                }
            }
            Hir {
                kind: HirKind::Alternation(subs),
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }