fn concat(concat: &[Hir]) -> Properties {
        // The base case is an empty concatenation, which matches the empty
        // string. Note though that empty concatenations aren't possible,
        // because the Hir::concat smart constructor rewrites those as
        // Hir::empty.
        let mut props = PropertiesI {
            minimum_len: Some(0),
            maximum_len: Some(0),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        };
        // Handle properties that need to visit every child hir.
        for x in concat.iter() {
            let p = x.properties();
            props.look_set.set_union(p.look_set());
            props.utf8 = props.utf8 && p.is_utf8();
            props.explicit_captures_len = props
                .explicit_captures_len
                .saturating_add(p.explicit_captures_len());
            props.static_explicit_captures_len = p
                .static_explicit_captures_len()
                .and_then(|len1| {
                    Some((len1, props.static_explicit_captures_len?))
                })
                .and_then(|(len1, len2)| Some(len1.saturating_add(len2)));
            props.literal = props.literal && p.is_literal();
            props.alternation_literal =
                props.alternation_literal && p.is_alternation_literal();
            if let Some(minimum_len) = props.minimum_len {
                match p.minimum_len() {
                    None => props.minimum_len = None,
                    Some(len) => {
                        // We use saturating arithmetic here because the
                        // minimum is just a lower bound. We can't go any
                        // higher than what our number types permit.
                        props.minimum_len =
                            Some(minimum_len.saturating_add(len));
                    }
                }
            }
            if let Some(maximum_len) = props.maximum_len {
                match p.maximum_len() {
                    None => props.maximum_len = None,
                    Some(len) => {
                        props.maximum_len = maximum_len.checked_add(len)
                    }
                }
            }
        }
        // Handle the prefix properties, which only requires visiting
        // child exprs until one matches more than the empty string.
        let mut it = concat.iter();
        while let Some(x) = it.next() {
            props.look_set_prefix.set_union(x.properties().look_set_prefix());
            props
                .look_set_prefix_any
                .set_union(x.properties().look_set_prefix_any());
            if x.properties().maximum_len().map_or(true, |x| x > 0) {
                break;
            }
        }
        // Same thing for the suffix properties, but in reverse.
        let mut it = concat.iter().rev();
        while let Some(x) = it.next() {
            props.look_set_suffix.set_union(x.properties().look_set_suffix());
            props
                .look_set_suffix_any
                .set_union(x.properties().look_set_suffix_any());
            if x.properties().maximum_len().map_or(true, |x| x > 0) {
                break;
            }
        }
        Properties(Box::new(props))
    }