fn class(class: &Class) -> Properties {
        let inner = PropertiesI {
            minimum_len: class.minimum_len(),
            maximum_len: class.maximum_len(),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: class.is_utf8(),
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: false,
            alternation_literal: false,
        };
        Properties(Box::new(inner))
    }