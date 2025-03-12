// Answer 0

#[test]
fn test_is_canonical_with_duplicate_ranges() {
    let range1 = ClassRange { start: 'a', end: 'a' };
    let range2 = ClassRange { start: 'a', end: 'a' };
    let class = Class::new(vec![range1, range2]);
    class.is_canonical();
}

#[test]
fn test_is_canonical_with_overlapping_ranges() {
    let range1 = ClassRange { start: 'a', end: 'b' };
    let range2 = ClassRange { start: 'b', end: 'c' };
    let class = Class::new(vec![range1, range2]);
    class.is_canonical();
}

