// Answer 0

#[test]
fn test_negate_with_empty_ranges() {
    let mut class = Class::new(vec![ClassRange { start: '\x00', end: char::MAX }]);
    class.negate();
}

#[test]
fn test_negate_with_start_equals_min() {
    let mut class = Class::new(vec![ClassRange { start: '\x00', end: 'a' }]);
    class.negate();
}

#[test]
fn test_negate_with_no_ranges_beyond_first() {
    let mut class = Class::new(vec![ClassRange { start: 'a', end: 'b' }]);
    class.negate();
}

#[test]
fn test_negate_with_end_equals_max() {
    let mut class = Class::new(vec![ClassRange { start: 'a', end: char::MAX }]);
    class.negate();
}

#[test]
fn test_negate_with_multiple_ranges_ending_at_max() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'c', end: char::MAX },
    ]);
    class.negate();
}

