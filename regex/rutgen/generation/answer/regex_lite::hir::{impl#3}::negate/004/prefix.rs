// Answer 0

#[test]
fn test_negate_with_single_range() {
    let mut class = Class::new(vec![ClassRange { start: 'a', end: 'z' }]);
    class.negate();
}

#[test]
fn test_negate_with_multiple_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'g', end: 'j' },
    ]);
    class.negate();
} 

#[test]
fn test_negate_with_overlapping_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'e' },
        ClassRange { start: 'g', end: 'k' },
    ]);
    class.negate();
} 

#[test]
fn test_negate_with_contiguous_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'd', end: 'g' },
        ClassRange { start: 'i', end: 'm' },
    ]);
    class.negate();
} 

#[test]
fn test_negate_with_full_range() {
    let mut class = Class::new(vec![
        ClassRange { start: 'd', end: 'x' },
    ]);
    class.negate();
}

