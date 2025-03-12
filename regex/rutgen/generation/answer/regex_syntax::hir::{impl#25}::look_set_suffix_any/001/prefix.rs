// Answer 0

#[test]
fn test_look_set_suffix_any_with_none_minimum_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: Some(5),
        look_set: LookSet { bits: 0 },
        look_set_prefix: LookSet { bits: 0 },
        look_set_suffix: LookSet { bits: 0 },
        look_set_prefix_any: LookSet { bits: 0 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_zero_minimum_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: Some(0),
        maximum_len: Some(5),
        look_set: LookSet { bits: 1 },
        look_set_prefix: LookSet { bits: 1 },
        look_set_suffix: LookSet { bits: 1 },
        look_set_prefix_any: LookSet { bits: 1 },
        look_set_suffix_any: LookSet { bits: 0xAA55 },
        utf8: false,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(0),
        literal: true,
        alternation_literal: false,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_positive_minimum_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: Some(3),
        maximum_len: Some(10),
        look_set: LookSet { bits: 2 },
        look_set_prefix: LookSet { bits: 2 },
        look_set_suffix: LookSet { bits: 2 },
        look_set_prefix_any: LookSet { bits: 2 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: true,
        explicit_captures_len: 5,
        static_explicit_captures_len: Some(2),
        literal: false,
        alternation_literal: true,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_none_maximum_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: None,
        look_set: LookSet { bits: 3 },
        look_set_prefix: LookSet { bits: 3 },
        look_set_suffix: LookSet { bits: 3 },
        look_set_prefix_any: LookSet { bits: 3 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(3),
        literal: true,
        alternation_literal: false,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_zero_explicit_captures_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: Some(5),
        look_set: LookSet { bits: 4 },
        look_set_prefix: LookSet { bits: 4 },
        look_set_suffix: LookSet { bits: 4 },
        look_set_prefix_any: LookSet { bits: 4 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_positive_static_explicit_captures_len() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: Some(2),
        maximum_len: Some(5),
        look_set: LookSet { bits: 5 },
        look_set_prefix: LookSet { bits: 5 },
        look_set_suffix: LookSet { bits: 5 },
        look_set_prefix_any: LookSet { bits: 5 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: true,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(4),
        literal: false,
        alternation_literal: true,
    }));
    props.look_set_suffix_any();
}

#[test]
fn test_look_set_suffix_any_with_all_literal_set_to_true() {
    let props = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(10),
        look_set: LookSet { bits: 6 },
        look_set_prefix: LookSet { bits: 6 },
        look_set_suffix: LookSet { bits: 6 },
        look_set_prefix_any: LookSet { bits: 6 },
        look_set_suffix_any: LookSet { bits: 0xFFFF },
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: None,
        literal: true,
        alternation_literal: false,
    }));
    props.look_set_suffix_any();
}

