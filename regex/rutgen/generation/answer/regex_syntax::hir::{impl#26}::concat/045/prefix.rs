// Answer 0

#[test]
fn test_concat_with_valid_hir_properties() {
    struct TestHir {
        props: Properties,
    }

    impl TestHir {
        fn properties(&self) -> &Properties {
            &self.props
        }
    }

    let hir1 = TestHir {
        props: Properties(Box::new(PropertiesI {
            minimum_len: Some(1),
            maximum_len: Some(2),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::full(),
            look_set_suffix: LookSet::full(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        })),
    };

    let hir2 = TestHir {
        props: Properties(Box::new(PropertiesI {
            minimum_len: Some(1),
            maximum_len: Some(3),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::full(),
            look_set_suffix: LookSet::full(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        })),
    };

    let concat_hirs = vec![hir1, hir2];

    let _result = Properties::concat(&concat_hirs);
}

#[test]
fn test_concat_with_multiple_hir_properties() {
    struct TestHir {
        props: Properties,
    }

    impl TestHir {
        fn properties(&self) -> &Properties {
            &self.props
        }
    }

    let hir1 = TestHir {
        props: Properties(Box::new(PropertiesI {
            minimum_len: Some(2),
            maximum_len: Some(4),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::full(),
            look_set_suffix: LookSet::full(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        })),
    };

    let hir2 = TestHir {
        props: Properties(Box::new(PropertiesI {
            minimum_len: Some(1),
            maximum_len: Some(5),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::full(),
            look_set_suffix: LookSet::full(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        })),
    };

    let hir3 = TestHir {
        props: Properties(Box::new(PropertiesI {
            minimum_len: Some(1),
            maximum_len: Some(2),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::full(),
            look_set_suffix: LookSet::full(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: true,
            alternation_literal: true,
        })),
    };

    let concat_hirs = vec![hir1, hir2, hir3];

    let _result = Properties::concat(&concat_hirs);
}

