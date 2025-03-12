// Answer 0

#[test]
fn test_minimum_len_some_zero() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(0),
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
    properties.minimum_len();
}

#[test]
fn test_minimum_len_some_non_zero() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(42),
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: true,
        explicit_captures_len: 5,
        static_explicit_captures_len: Some(2),
        literal: true,
        alternation_literal: false,
    }));
    properties.minimum_len();
}

#[test]
fn test_minimum_len_none() {
    let properties = Properties(Box::new(PropertiesI {
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
    properties.minimum_len();
}

#[test]
fn test_minimum_len_lower_bound() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(1),
        literal: true,
        alternation_literal: false,
    }));
    properties.minimum_len();
}

#[test]
fn test_minimum_len_upper_bound() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(100),
        maximum_len: None,
        look_set: LookSet::default(),
        look_set_prefix: LookSet::default(),
        look_set_suffix: LookSet::default(),
        look_set_prefix_any: LookSet::default(),
        look_set_suffix_any: LookSet::default(),
        utf8: true,
        explicit_captures_len: 50,
        static_explicit_captures_len: Some(10),
        literal: false,
        alternation_literal: true,
    }));
    properties.minimum_len();
}

