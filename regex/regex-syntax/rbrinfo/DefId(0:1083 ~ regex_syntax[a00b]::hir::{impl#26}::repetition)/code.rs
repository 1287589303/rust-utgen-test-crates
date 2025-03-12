fn repetition(rep: &Repetition) -> Properties {
        let p = rep.sub.properties();
        let minimum_len = p.minimum_len().map(|child_min| {
            let rep_min = usize::try_from(rep.min).unwrap_or(usize::MAX);
            child_min.saturating_mul(rep_min)
        });
        let maximum_len = rep.max.and_then(|rep_max| {
            let rep_max = usize::try_from(rep_max).ok()?;
            let child_max = p.maximum_len()?;
            child_max.checked_mul(rep_max)
        });

        let mut inner = PropertiesI {
            minimum_len,
            maximum_len,
            look_set: p.look_set(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: p.look_set_prefix_any(),
            look_set_suffix_any: p.look_set_suffix_any(),
            utf8: p.is_utf8(),
            explicit_captures_len: p.explicit_captures_len(),
            static_explicit_captures_len: p.static_explicit_captures_len(),
            literal: false,
            alternation_literal: false,
        };
        // If the repetition operator can match the empty string, then its
        // lookset prefix and suffixes themselves remain empty since they are
        // no longer required to match.
        if rep.min > 0 {
            inner.look_set_prefix = p.look_set_prefix();
            inner.look_set_suffix = p.look_set_suffix();
        }
        // If the static captures len of the sub-expression is not known or
        // is greater than zero, then it automatically propagates to the
        // repetition, regardless of the repetition. Otherwise, it might
        // change, but only when the repetition can match 0 times.
        if rep.min == 0
            && inner.static_explicit_captures_len.map_or(false, |len| len > 0)
        {
            // If we require a match 0 times, then our captures len is
            // guaranteed to be zero. Otherwise, if we *can* match the empty
            // string, then it's impossible to know how many captures will be
            // in the resulting match.
            if rep.max == Some(0) {
                inner.static_explicit_captures_len = Some(0);
            } else {
                inner.static_explicit_captures_len = None;
            }
        }
        Properties(Box::new(inner))
    }