// Answer 0

#[test]
fn test_explicit_captures_len_no_groups() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    let _ = props.explicit_captures_len();
}

#[test]
fn test_explicit_captures_len_single_group() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    let _ = props.explicit_captures_len();
}

#[test]
fn test_explicit_captures_len_multiple_groups() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: false,
        explicit_captures_len: 2,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    let _ = props.explicit_captures_len();
}

#[test]
fn test_explicit_captures_len_nested_groups() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: false,
        explicit_captures_len: 3,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    let _ = props.explicit_captures_len();
}

#[test]
fn test_explicit_captures_len_empty_string() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    let _ = props.explicit_captures_len();
}

