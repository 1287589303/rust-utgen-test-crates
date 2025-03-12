// Answer 0

#[test]
fn test_repetition_with_min_greater_than_zero() {
    struct DummyHir;

    impl DummyHir {
        fn properties(&self) -> Properties {
            let look_set = LookSet::empty();
            Properties(Box::new(PropertiesI {
                minimum_len: Some(5),
                maximum_len: Some(10),
                look_set,
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: look_set,
                look_set_suffix_any: look_set,
                utf8: true,
                explicit_captures_len: 2,
                static_explicit_captures_len: Some(1),
                literal: false,
                alternation_literal: false,
            }))
        }
    }

    let repetition = Repetition {
        min: 1,
        max: Some(10),
        greedy: true,
        sub: Box::new(DummyHir),
    };

    let _ = Properties::repetition(&repetition);
}

#[test]
fn test_repetition_with_min_zero() {
    struct DummyHir;

    impl DummyHir {
        fn properties(&self) -> Properties {
            let look_set = LookSet::empty();
            Properties(Box::new(PropertiesI {
                minimum_len: Some(0),
                maximum_len: Some(5),
                look_set,
                look_set_prefix: LookSet::empty(),
                look_set_suffix: LookSet::empty(),
                look_set_prefix_any: look_set,
                look_set_suffix_any: look_set,
                utf8: false,
                explicit_captures_len: 1,
                static_explicit_captures_len: Some(2),
                literal: true,
                alternation_literal: false,
            }))
        }
    }

    let repetition = Repetition {
        min: 0,
        max: Some(5),
        greedy: false,
        sub: Box::new(DummyHir),
    };

    let _ = Properties::repetition(&repetition);
}

