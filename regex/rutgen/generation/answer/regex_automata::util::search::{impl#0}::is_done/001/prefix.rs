// Answer 0

#[test]
fn test_is_done_true_case() {
    let mut input = Input::new("example");
    input.set_start(8);
    input.set_end(7);
    assert!(input.is_done());
}

#[test]
fn test_is_done_false_case_equal() {
    let mut input = Input::new("example");
    input.set_start(5);
    input.set_end(5);
    assert!(!input.is_done());
}

#[test]
fn test_is_done_false_case_less() {
    let mut input = Input::new("example");
    input.set_start(3);
    input.set_end(5);
    assert!(!input.is_done());
}

#[test]
fn test_is_done_with_boundary_cases() {
    let mut input = Input::new("test");
    input.set_start(0);
    input.set_end(1);
    assert!(!input.is_done());

    input.set_start(1);
    input.set_end(0);
    assert!(input.is_done());
}

