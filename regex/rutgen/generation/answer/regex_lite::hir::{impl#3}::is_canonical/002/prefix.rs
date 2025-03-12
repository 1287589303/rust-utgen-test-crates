// Answer 0

#[test]
fn test_is_canonical_false_case() {
    let range1 = ClassRange { start: 'a', end: 'd' };
    let range2 = ClassRange { start: 'c', end: 'e' };
    let class = Class::new(vec![range1, range2]);
    let _ = class.is_canonical();
}

#[test]
fn test_is_canonical_false_case_boundary() {
    let range1 = ClassRange { start: 'x', end: 'z' };
    let range2 = ClassRange { start: 'z', end: 'a' }; // Note: `z` is the end of range1, creating overlap
    let class = Class::new(vec![range1, range2]);
    let _ = class.is_canonical();
}

