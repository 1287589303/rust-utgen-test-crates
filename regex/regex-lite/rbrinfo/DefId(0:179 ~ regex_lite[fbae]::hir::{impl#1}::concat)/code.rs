fn concat(mut subs: Vec<Hir>) -> Hir {
        if subs.is_empty() {
            Hir::empty()
        } else if subs.len() == 1 {
            subs.pop().unwrap()
        } else {
            let is_start_anchored = subs[0].is_start_anchored;
            let mut is_match_empty = true;
            let mut static_explicit_captures_len = Some(0usize);
            for sub in subs.iter() {
                is_match_empty = is_match_empty && sub.is_match_empty;
                static_explicit_captures_len = static_explicit_captures_len
                    .and_then(|len1| {
                        Some((len1, sub.static_explicit_captures_len?))
                    })
                    .and_then(|(len1, len2)| Some(len1.saturating_add(len2)));
            }
            Hir {
                kind: HirKind::Concat(subs),
                is_start_anchored,
                is_match_empty,
                static_explicit_captures_len,
            }
        }
    }