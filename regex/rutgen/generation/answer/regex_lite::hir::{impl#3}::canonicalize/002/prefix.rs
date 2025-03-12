// Answer 0

#[test]
fn test_canonicalize_non_canonical_with_empty_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'a', end: 'c' }, 
        ClassRange { start: 'd', end: 'f' }
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_with_multiple_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'm', end: 'n' }, 
        ClassRange { start: 'a', end: 'b' }, 
        ClassRange { start: 'c', end: 'e' }
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_with_contiguous_ranges() {
    let mut class = Class::new(vec![
        ClassRange { start: 'x', end: 'y' }, 
        ClassRange { start: 'y', end: 'z' }
    ]);
    class.canonicalize();
}

#[test]
fn test_canonicalize_non_canonical_with_overlap() {
    let mut class = Class::new(vec![
        ClassRange { start: 'h', end: 'l' }, 
        ClassRange { start: 'j', end: 'n' }
    ]);
    class.canonicalize();
}

