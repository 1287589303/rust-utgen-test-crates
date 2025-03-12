// Answer 0

#[test]
fn test_range_valid_case() {
    let text = "Hello, World!";
    let match_instance = Match::new(text, 7, 12);
    let _result = match_instance.range();
}

#[test]
fn test_range_boundary_case_start_zero() {
    let text = "Boundary Test";
    let match_instance = Match::new(text, 0, 8);
    let _result = match_instance.range();
}

#[test]
fn test_range_boundary_case_end_equal_length() {
    let text = "Final Test";
    let match_instance = Match::new(text, 5, 10);
    let _result = match_instance.range();
}

#[test]
fn test_range_non_empty_case() {
    let text = "Rust Programming";
    let match_instance = Match::new(text, 0, 4);
    let _result = match_instance.range();
}

#[test]
#[should_panic]
fn test_range_invalid_case_start_greater_than_end() {
    let text = "Invalid Case";
    let match_instance = Match::new(text, 5, 3);
    let _result = match_instance.range();
}

