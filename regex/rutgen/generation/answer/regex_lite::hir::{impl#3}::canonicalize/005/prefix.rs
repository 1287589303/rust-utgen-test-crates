// Answer 0

#[test]
fn test_canonicalize_non_empty_non_canonical_overlapping_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'b', end: 'd' },
        ClassRange { start: 'e', end: 'g' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_empty_non_canonical_contiguous_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'b', end: 'c' },
        ClassRange { start: 'd', end: 'f' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_empty_non_canonical_disjoint_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'a' },
        ClassRange { start: 'c', end: 'c' },
        ClassRange { start: 'd', end: 'd' },
    ]);
    class.canonicalize();
}

