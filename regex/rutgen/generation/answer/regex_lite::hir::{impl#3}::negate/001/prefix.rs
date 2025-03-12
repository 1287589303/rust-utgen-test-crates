// Answer 0

#[test]
fn test_negate_empty_ranges() {
    let mut class = Class::new(vec![]);
    class.negate();
}

#[test]
fn test_negate_single_class_range() {
    let mut class = Class::new(vec![ClassRange { start: '\x00', end: '\x00' }]);
    class.negate();
}

