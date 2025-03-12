// Answer 0

#[test]
fn test_concat_properties_with_none_lengths() {
    struct TestHir {
        props: Properties,
    }

    impl TestHir {
        fn properties(&self) -> &Properties {
            &self.props
        }
    }

    let empty_look_set = LookSet::empty();
    let prop_none = Properties(Box::new(PropertiesI {
        minimum_len: None,
        maximum_len: None,
        look_set: empty_look_set,
        look_set_prefix: empty_look_set,
        look_set_suffix: empty_look_set,
        look_set_prefix_any: empty_look_set,
        look_set_suffix_any: empty_look_set,
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(0),
        literal: false,
        alternation_literal: false,
    }));

    let hir1 = TestHir { props: prop_none.clone() };
    let hir2 = TestHir { props: prop_none.clone() };
    let concatenated_props = Properties::concat(&[hir1, hir2]);

    let _ = concatenated_props; // This is just to satisfy the function call.
}

#[test]
fn test_concat_properties_with_zero_lengths() {
    struct TestHir {
        props: Properties,
    }

    impl TestHir {
        fn properties(&self) -> &Properties {
            &self.props
        }
    }

    let empty_look_set = LookSet::empty();
    let prop_zero = Properties(Box::new(PropertiesI {
        minimum_len: Some(0),
        maximum_len: Some(0),
        look_set: empty_look_set,
        look_set_prefix: empty_look_set,
        look_set_suffix: empty_look_set,
        look_set_prefix_any: empty_look_set,
        look_set_suffix_any: empty_look_set,
        utf8: false,
        explicit_captures_len: 0,
        static_explicit_captures_len: Some(0),
        literal: false,
        alternation_literal: false,
    }));

    let hir1 = TestHir { props: prop_zero.clone() };
    let hir2 = TestHir { props: prop_zero.clone() };
    let concatenated_props = Properties::concat(&[hir1, hir2]);

    let _ = concatenated_props; // This is just to satisfy the function call.
}

