// Answer 0

#[test]
fn test_union_properties_with_poisoned_min_max() {
    let props1 = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(2),
        literal: false,
        alternation_literal: false,
    }));
    
    let props2 = Properties(Box::new(PropertiesI {
        minimum_len: Some(3),
        maximum_len: Some(5),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 2,
        static_explicit_captures_len: Some(2),
        literal: false,
        alternation_literal: false,
    }));

    let unioned = Properties::union(vec![&props1, &props2]);
}

#[test]
fn test_union_properties_with_mixed_poisoning() {
    let props1 = Properties(Box::new(PropertiesI {
        minimum_len: Some(2),
        maximum_len: Some(4),
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 3,
        static_explicit_captures_len: Some(3),
        literal: false,
        alternation_literal: false,
    }));

    let props2 = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 3,
        static_explicit_captures_len: Some(3),
        literal: false,
        alternation_literal: false,
    }));

    let props3 = Properties(Box::new(PropertiesI {
        minimum_len: Some(5),
        maximum_len: None,
        look_set: LookSet::empty(),
        look_set_prefix: LookSet::full(),
        look_set_suffix: LookSet::full(),
        look_set_prefix_any: LookSet::empty(),
        look_set_suffix_any: LookSet::empty(),
        utf8: false,
        explicit_captures_len: 3,
        static_explicit_captures_len: Some(3),
        literal: false,
        alternation_literal: false,
    }));

    let unioned = Properties::union(vec![&props1, &props2, &props3]);
}

