// Answer 0

#[test]
fn test_look_set_suffix_none() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet { bits: 0 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 0 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0 },
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: false,
    }));
    properties.look_set_suffix();
}

#[test]
fn test_look_set_suffix_minimum_len() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(5),
        maximum_len: None,
        look_set: LookSet { bits: 0 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 1 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0 },
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: true,
        alternation_literal: false,
    }));
    properties.look_set_suffix();
}

#[test]
fn test_look_set_suffix_maximum_len() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: Some(10),
        look_set: LookSet { bits: 2 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 0 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0 },
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));
    properties.look_set_suffix();
}

#[test]
fn test_look_set_suffix_full_range() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet { bits: u32::MAX },
        look_set_prefix: LookSet { bits: u32::MAX },
        look_set_suffix: LookSet { bits: u32::MAX },
        look_set_prefix_any: LookSet { bits: u32::MAX },
        look_set_suffix_any: LookSet { bits: u32::MAX },
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: true,
        alternation_literal: true,
    }));
    properties.look_set_suffix();
} 

#[test]
fn test_look_set_suffix_variant() {
    let properties = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(5),
        look_set: LookSet { bits: 8 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 4 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0 },
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(2),
        literal: true,
        alternation_literal: false,
    }));
    properties.look_set_suffix();
}

