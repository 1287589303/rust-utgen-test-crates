fn from_nfa(nfa: &thompson::NFA) -> Flags {
        Flags {
            has_empty: nfa.has_empty(),
            is_utf8: nfa.is_utf8(),
            is_always_start_anchored: nfa.is_always_start_anchored(),
        }
    }