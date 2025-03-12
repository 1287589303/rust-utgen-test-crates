// Answer 0

#[test]
fn test_matches_b_less_than_start() {
    let range = Utf8Range::new(100, 200);
    let result = range.matches(50);
}

#[test]
fn test_matches_b_less_than_start_at_lower_bound() {
    let range = Utf8Range::new(1, 255);
    let result = range.matches(0);
}

#[test]
fn test_matches_b_less_than_start_at_start() {
    let range = Utf8Range::new(10, 20);
    let result = range.matches(9);
}

#[test]
fn test_matches_b_less_than_start_with_max_values() {
    let range = Utf8Range::new(255, 255);
    let result = range.matches(254);
}

