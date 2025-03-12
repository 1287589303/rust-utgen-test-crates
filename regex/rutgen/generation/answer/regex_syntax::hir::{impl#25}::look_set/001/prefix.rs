// Answer 0

#[test]
fn test_look_set_with_minimum_len_zero() {
    let properties_i = PropertiesI {
        minimum_len: Some(0),
        maximum_len: Some(100),
        look_set: LookSet { bits: 1 },
        look_set_prefix: LookSet { bits: 2 },
        look_set_suffix: LookSet { bits: 3 },
        look_set_prefix_any: LookSet { bits: 4 },
        look_set_suffix_any: LookSet { bits: 5 },
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(0),
        literal: false,
        alternation_literal: false,
    };
    let properties = Properties(Box::new(properties_i));
    properties.look_set();
}

#[test]
fn test_look_set_with_minimum_len_fifty() {
    let properties_i = PropertiesI {
        minimum_len: Some(50),
        maximum_len: Some(100),
        look_set: LookSet { bits: 255 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 128 },
        look_set_prefix_any: LookSet { bits: 64 },
        look_set_suffix_any: LookSet { bits: 32 },
        utf8: false,
        explicit_captures_len: 5,
        static_explicit_captures_len: Some(2),
        literal: true,
        alternation_literal: true,
    };
    let properties = Properties(Box::new(properties_i));
    properties.look_set();
}

#[test]
fn test_look_set_with_maximum_len_hundred() {
    let properties_i = PropertiesI {
        minimum_len: Some(100),
        maximum_len: Some(100),
        look_set: LookSet { bits: 15 },
        look_set_prefix: LookSet { bits: 10 },
        look_set_suffix: LookSet { bits: 5 },
        look_set_prefix_any: LookSet { bits: 3 },
        look_set_suffix_any: LookSet { bits: 1 },
        utf8: true,
        explicit_captures_len: 3,
        static_explicit_captures_len: None,
        literal: true,
        alternation_literal: false,
    };
    let properties = Properties(Box::new(properties_i));
    properties.look_set();
}

#[test]
fn test_look_set_with_literally_false() {
    let properties_i = PropertiesI {
        minimum_len: Some(10),
        maximum_len: Some(80),
        look_set: LookSet { bits: 30 },
        look_set_prefix: LookSet { bits: 30 },
        look_set_suffix: LookSet { bits: 30 },
        look_set_prefix_any: LookSet { bits: 30 },
        look_set_suffix_any: LookSet { bits: 30 },
        utf8: false,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    };
    let properties = Properties(Box::new(properties_i));
    properties.look_set();
}

#[test]
fn test_look_set_with_utf8_false_and_minimum_len_hundred() {
    let properties_i = PropertiesI {
        minimum_len: Some(100),
        maximum_len: Some(100),
        look_set: LookSet { bits: 0 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 0 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0 },
        utf8: false,
        explicit_captures_len: 4,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    };
    let properties = Properties(Box::new(properties_i));
    properties.look_set();
}

