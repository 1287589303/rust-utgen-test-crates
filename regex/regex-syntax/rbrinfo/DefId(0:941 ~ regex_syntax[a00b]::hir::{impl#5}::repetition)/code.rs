pub fn repetition(mut rep: Repetition) -> Hir {
        // If the sub-expression of a repetition can only match the empty
        // string, then we force its maximum to be at most 1.
        if rep.sub.properties().maximum_len() == Some(0) {
            rep.min = cmp::min(rep.min, 1);
            rep.max = rep.max.map(|n| cmp::min(n, 1)).or(Some(1));
        }
        // The regex 'a{0}' is always equivalent to the empty regex. This is
        // true even when 'a' is an expression that never matches anything
        // (like '\P{any}').
        //
        // Additionally, the regex 'a{1}' is always equivalent to 'a'.
        if rep.min == 0 && rep.max == Some(0) {
            return Hir::empty();
        } else if rep.min == 1 && rep.max == Some(1) {
            return *rep.sub;
        }
        let props = Properties::repetition(&rep);
        Hir { kind: HirKind::Repetition(rep), props }
    }