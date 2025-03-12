// Answer 0

#[test]
#[should_panic]
fn test_new_match_with_start_greater_than_end() {
    let pattern = PatternID(0);
    let span = Span { start: 5, end: 4 };
    Match::new(pattern, span);
}

#[test]
#[should_panic]
fn test_new_match_with_negative_span() {
    let pattern = PatternID(1);
    let span = Span { start: 3, end: 2 };
    Match::new(pattern, span);
}

#[test]
#[should_panic]
fn test_new_match_with_equal_start_end() {
    let pattern = PatternID(2);
    let span = Span { start: 6, end: 6 };
    Match::new(pattern, span);
}

