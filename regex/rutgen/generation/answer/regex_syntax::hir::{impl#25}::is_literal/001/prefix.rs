// Answer 0

#[test]
fn test_is_literal_true() {
    struct TestProperties {
        minimum_len: Option<usize>,
        maximum_len: Option<usize>,
        literal: bool,
    }

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
        literal: true,
        alternation_literal: false,
    }));

    let result = props.is_literal();
}

#[test]
fn test_is_literal_false() {
    struct TestProperties {
        minimum_len: Option<usize>,
        maximum_len: Option<usize>,
        literal: bool,
    }

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

    let result = props.is_literal();
}

