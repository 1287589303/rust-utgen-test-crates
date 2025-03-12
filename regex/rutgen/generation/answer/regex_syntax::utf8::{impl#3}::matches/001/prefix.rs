// Answer 0

#[test]
fn matches_boundary_start() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(10);
}

#[test]
fn matches_boundary_end() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(20);
}

#[test]
fn matches_middle_value() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(15);
}

#[test]
fn matches_value_below_start() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(9);
}

#[test]
fn matches_value_above_end() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(21);
}

#[test]
fn matches_start_equals_end() {
    let range = Utf8Range::new(10, 10);
    let result = range.matches(10);
}

#[test]
fn matches_value_equal_to_start_minus_one() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(9);
}

#[test]
fn matches_value_equal_to_end_plus_one() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(21);
}

