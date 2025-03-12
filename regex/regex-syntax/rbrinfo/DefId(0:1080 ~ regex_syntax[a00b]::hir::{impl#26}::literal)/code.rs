fn literal(lit: &Literal) -> Properties {
        let inner = PropertiesI {
            minimum_len: Some(lit.0.len()),
            maximum_len: Some(lit.0.len()),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: core::str::from_utf8(&lit.0).is_ok(),
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        };
        Properties(Box::new(inner))
    }