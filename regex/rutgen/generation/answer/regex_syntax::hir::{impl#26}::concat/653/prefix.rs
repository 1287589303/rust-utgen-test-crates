// Answer 0

#[test]
fn test_concat_non_empty_hir_with_max_length() {
    struct MockHir {
        props: Properties,
    }
    
    impl MockHir {
        fn new(max_length: usize, is_utf8: bool) -> Self {
            let properties = Properties(Box::new(PropertiesI {
                minimum_len: Some(1),
                maximum_len: Some(max_length),
                look_set: LookSet::empty(),
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: LookSet::empty(),
                look_set_suffix_any: LookSet::empty(),
                utf8: is_utf8,
                explicit_captures_len: 0,
                static_explicit_captures_len: Some(0),
                literal: true,
                alternation_literal: true,
            }));
            Self { props: properties }
        }
    }

    let hirs = [
        MockHir::new(2, true).props,
        MockHir::new(3, true).props,
        MockHir::new(1, true).props,
    ];

    Properties::concat(&hirs);
}

#[test]
fn test_concat_multiple_hir_with_max_length_non_zero() {
    struct MockHir {
        props: Properties,
    }
    
    impl MockHir {
        fn new(max_length: usize, is_utf8: bool) -> Self {
            let properties = Properties(Box::new(PropertiesI {
                minimum_len: Some(1),
                maximum_len: Some(max_length),
                look_set: LookSet::empty(),
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: LookSet::empty(),
                look_set_suffix_any: LookSet::empty(),
                utf8: is_utf8,
                explicit_captures_len: 0,
                static_explicit_captures_len: Some(0),
                literal: true,
                alternation_literal: true,
            }));
            Self { props: properties }
        }
    }

    let hirs = [
        MockHir::new(5, true).props,
        MockHir::new(10, true).props,
    ];

    Properties::concat(&hirs);
}

#[test]
fn test_concat_single_hir_with_max_length() {
    struct MockHir {
        props: Properties,
    }
    
    impl MockHir {
        fn new(max_length: usize, is_utf8: bool) -> Self {
            let properties = Properties(Box::new(PropertiesI {
                minimum_len: Some(1),
                maximum_len: Some(max_length),
                look_set: LookSet::empty(),
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: LookSet::empty(),
                look_set_suffix_any: LookSet::empty(),
                utf8: is_utf8,
                explicit_captures_len: 0,
                static_explicit_captures_len: Some(0),
                literal: true,
                alternation_literal: true,
            }));
            Self { props: properties }
        }
    }

    let hirs = [
        MockHir::new(4, true).props,
    ];

    Properties::concat(&hirs);
}

