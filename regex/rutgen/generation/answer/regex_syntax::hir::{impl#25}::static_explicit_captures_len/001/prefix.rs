// Answer 0

#[test]
fn test_static_explicit_captures_len_case_1() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(0),
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_2() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_3() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_4() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(2),
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_5() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_6() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    properties.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_case_7() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::new(),
        look_set_prefix: LookSet::new(),
        look_set_suffix: LookSet::new(),
        look_set_prefix_any: LookSet::new(),
        look_set_suffix_any: LookSet::new(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));
    properties.static_explicit_captures_len();
}

