// Answer 0

#[test]
fn test_canonicalize_non_empty_ranges_with_non_overlapping_elements() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'd', end: 'e' },
        ClassRange { start: 'g', end: 'h' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_empty_ranges_with_overlapping_elements() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'b', end: 'f' },
        ClassRange { start: 'g', end: 'j' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_empty_ranges_with_edge_characters() {
    let mut class = Class::new(vec![
        ClassRange { start: '0', end: '9' },
        ClassRange { start: 'A', end: 'Z' },
        ClassRange { start: 'a', end: 'z' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_with_gaps_between_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'd', end: 'e' },
        ClassRange { start: 'g', end: 'j' },
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_with_special_characters() {
    let mut class = Class::new(vec![
        ClassRange { start: '!', end: '@' },
        ClassRange { start: '#', end: '$' },
        ClassRange { start: '%', end: '&' },
    ]);
    class.canonicalize();
}

