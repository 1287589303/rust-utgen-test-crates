// Answer 0

#[test]
fn test_canonicalize_non_canonical_ranges_with_merge() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'd' },
        ClassRange { start: 'c', end: 'f' },
        ClassRange { start: 'g', end: 'j' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_ranges_with_multiple_merges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'd' },
        ClassRange { start: 'b', end: 'e' },
        ClassRange { start: 'f', end: 'h' },
        ClassRange { start: 'g', end: 'j' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_ranges_with_edge_case() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'b', end: 'c' },
        ClassRange { start: 'd', end: 'g' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_with_empty_range_if_not_canonical() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'c', end: 'e' },
        ClassRange { start: 'e', end: 'f' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_with_high_characters() {
    let mut class = Class::new(vec![
        ClassRange { start: 'x', end: 'z' },
        ClassRange { start: 'y', end: 'a' },
    ]);
    class.canonicalize();
}

