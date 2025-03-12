// Answer 0

#[test]
fn test_negate_empty_ranges() {
    let mut class = Class::new(vec![]);
    class.negate();
}

#[test]
fn test_negate_first_range_start_min() {
    let mut class = Class::new(vec![ClassRange { start: '\x00', end: 'a' }]);
    class.negate();
}

#[test]
fn test_negate_single_range() {
    let mut class = Class::new(vec![ClassRange { start: 'a', end: 'b' }]);
    class.negate();
}

#[test]
fn test_negate_multiple_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'e', end: 'g' },
    ]);
    class.negate();
}

#[test]
fn test_negate_last_range_end_max() {
    let mut class = Class::new(vec![
        ClassRange { start: '\x00', end: 'a' },
        ClassRange { start: 'b', end: char::MAX },
    ]);
    class.negate();
}

