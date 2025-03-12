pub fn union<I, P>(props: I) -> Properties
    where
        I: IntoIterator<Item = P>,
        P: core::borrow::Borrow<Properties>,
    {
        let mut it = props.into_iter().peekable();
        // While empty alternations aren't possible, we still behave as if they
        // are. When we have an empty alternate, then clearly the look-around
        // prefix and suffix is empty. Otherwise, it is the intersection of all
        // prefixes and suffixes (respectively) of the branches.
        let fix = if it.peek().is_none() {
            LookSet::empty()
        } else {
            LookSet::full()
        };
        // And also, an empty alternate means we have 0 static capture groups,
        // but we otherwise start with the number corresponding to the first
        // alternate. If any subsequent alternate has a different number of
        // static capture groups, then we overall have a variation and not a
        // static number of groups.
        let static_explicit_captures_len =
            it.peek().and_then(|p| p.borrow().static_explicit_captures_len());
        // The base case is an empty alternation, which matches nothing.
        // Note though that empty alternations aren't possible, because the
        // Hir::alternation smart constructor rewrites those as empty character
        // classes.
        let mut props = PropertiesI {
            minimum_len: None,
            maximum_len: None,
            look_set: LookSet::empty(),
            look_set_prefix: fix,
            look_set_suffix: fix,
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len,
            literal: false,
            alternation_literal: true,
        };
        let (mut min_poisoned, mut max_poisoned) = (false, false);
        // Handle properties that need to visit every child hir.
        for prop in it {
            let p = prop.borrow();
            props.look_set.set_union(p.look_set());
            props.look_set_prefix.set_intersect(p.look_set_prefix());
            props.look_set_suffix.set_intersect(p.look_set_suffix());
            props.look_set_prefix_any.set_union(p.look_set_prefix_any());
            props.look_set_suffix_any.set_union(p.look_set_suffix_any());
            props.utf8 = props.utf8 && p.is_utf8();
            props.explicit_captures_len = props
                .explicit_captures_len
                .saturating_add(p.explicit_captures_len());
            if props.static_explicit_captures_len
                != p.static_explicit_captures_len()
            {
                props.static_explicit_captures_len = None;
            }
            props.alternation_literal =
                props.alternation_literal && p.is_literal();
            if !min_poisoned {
                if let Some(xmin) = p.minimum_len() {
                    if props.minimum_len.map_or(true, |pmin| xmin < pmin) {
                        props.minimum_len = Some(xmin);
                    }
                } else {
                    props.minimum_len = None;
                    min_poisoned = true;
                }
            }
            if !max_poisoned {
                if let Some(xmax) = p.maximum_len() {
                    if props.maximum_len.map_or(true, |pmax| xmax > pmax) {
                        props.maximum_len = Some(xmax);
                    }
                } else {
                    props.maximum_len = None;
                    max_poisoned = true;
                }
            }
        }
        Properties(Box::new(props))
    }