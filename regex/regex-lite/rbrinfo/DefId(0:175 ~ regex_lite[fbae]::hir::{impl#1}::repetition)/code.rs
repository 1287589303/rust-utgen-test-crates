fn repetition(rep: Repetition) -> Hir {
        if rep.min == 0 && rep.max == Some(0) {
            return Hir::empty();
        } else if rep.min == 1 && rep.max == Some(1) {
            return *rep.sub;
        }
        let is_start_anchored = rep.min > 0 && rep.sub.is_start_anchored;
        let is_match_empty = rep.min == 0 || rep.sub.is_match_empty;
        let mut static_explicit_captures_len =
            rep.sub.static_explicit_captures_len;
        // If the static captures len of the sub-expression is not known or
        // is greater than zero, then it automatically propagates to the
        // repetition, regardless of the repetition. Otherwise, it might
        // change, but only when the repetition can match 0 times.
        if rep.min == 0
            && static_explicit_captures_len.map_or(false, |len| len > 0)
        {
            // If we require a match 0 times, then our captures len is
            // guaranteed to be zero. Otherwise, if we *can* match the empty
            // string, then it's impossible to know how many captures will be
            // in the resulting match.
            if rep.max == Some(0) {
                static_explicit_captures_len = Some(0);
            } else {
                static_explicit_captures_len = None;
            }
        }
        Hir {
            kind: HirKind::Repetition(rep),
            is_start_anchored,
            is_match_empty,
            static_explicit_captures_len,
        }
    }