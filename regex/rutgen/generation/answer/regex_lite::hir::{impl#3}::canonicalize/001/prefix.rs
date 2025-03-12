// Answer 0

#[test]
fn test_canonicalize_already_canonical_single_range() {
    let mut class = Class::new(vec![ClassRange { start: 'a', end: 'b' }]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_already_canonical_multiple_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'c', end: 'd' },
        ClassRange { start: 'e', end: 'f' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_already_canonical_contiguous_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'b', end: 'c' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_already_canonical_with_gaps() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'd', end: 'f' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_already_canonical_reversed_by_end() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'c', end: 'd' },
        ClassRange { start: 'e', end: 'f' },
    ]);
    class.ranges.reverse();
    class.canonicalize();
}

