// Answer 0

#[test]
fn test_union_with_varied_properties() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(3),
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(1),
        literal: true,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: Some(2),
        maximum_len: Some(4),
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(2),
        literal: true,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2]);
}

#[test]
fn test_union_with_different_static_captures() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(3),
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: Some(1),
        literal: true,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: Some(3),
        maximum_len: Some(5),
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(3), // Different static captures length
        literal: true,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2]);
}

#[test]
fn test_union_with_min_max_conflict() {
    let prop1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(1),
        maximum_len: Some(4),
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(1),
        literal: true,
        alternation_literal: true,
    }));

    let prop2 = Properties(Box::new(PropertiesI {
        minimum_len: Some(2),
        maximum_len: Some(3), // Will lead to min being limited
        look_set: LookSet::full(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: true,
        explicit_captures_len: 1,
        static_explicit_captures_len: None, // Different static captures length
        literal: true,
        alternation_literal: true,
    }));

    let unioned = Properties::union(vec![&prop1, &prop2]);
}

