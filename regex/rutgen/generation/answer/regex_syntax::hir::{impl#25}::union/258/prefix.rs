// Answer 0

#[test]
fn test_union_with_non_empty_properties_poison_min_length() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(2),
        maximum_len: Some(4),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));

    let prop3 = Properties(Box::new(PropertiesI {
        minimum_len: Some(3),
        maximum_len: Some(5),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(2),
        literal: false,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2, &prop3]);
}

#[test]
fn test_union_with_non_empty_properties_poison_max_length() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(3),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    }));

    let prop3 = Properties(Box::new(PropertiesI {
        minimum_len: Some(3),
        maximum_len: Some(6),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2, &prop3]);
}

#[test]
fn test_union_with_all_properties_poisoned() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: false,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: None,
        literal: false,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2]);
}

