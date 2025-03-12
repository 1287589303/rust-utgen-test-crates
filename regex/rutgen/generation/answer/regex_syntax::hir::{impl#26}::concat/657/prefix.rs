// Answer 0

#[test]
fn test_concat_with_mixed_maximum_len() {
    struct DummyHir {
        props: Properties,
    }
    
    impl DummyHir {
        fn new(max_len: Option<usize>, literal: bool) -> Self {
            let look_set = LookSet::empty();
            let props = Properties(Box::new(PropertiesI {
                minimum_len: Some(1),
                maximum_len: max_len,
                look_set,
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: LookSet::empty(),
                look_set_suffix_any: LookSet::empty(),
                utf8: true,
                explicit_captures_len: 0,
                static_explicit_captures_len: Some(0),
                literal,
                alternation_literal: true,
            }));
            DummyHir { props }
        }

        fn properties(&self) -> &Properties {
            &self.props
        }
    }

    let hir_with_max_len_positive = DummyHir::new(Some(5), true); // max_len > 0
    let hir_with_max_len_zero = DummyHir::new(Some(0), true); // max_len <= 0
    let hir_with_max_len_none = DummyHir::new(None, false); // max_len = None

    let concat: Vec<DummyHir> = vec![
        hir_with_max_len_positive,
        hir_with_max_len_zero,
        hir_with_max_len_none,
    ];

    let _result = Properties::concat(&concat.iter().map(|h| h.properties()).collect::<Vec<_>>());
}

