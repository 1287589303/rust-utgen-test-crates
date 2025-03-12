// Answer 0

#[test]
fn test_is_canonical_with_non_overlapping_ranges() {
    let class = Class::new(vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'c', end: 'd' },
    ]);
    class.is_canonical();
}

#[test]
fn test_is_canonical_with_non_adjacent_ranges() {
    let class = Class::new(vec![
        ClassRange { start: 'e', end: 'f' },
        ClassRange { start: 'g', end: 'h' },
    ]);
    class.is_canonical();
}

#[test]
fn test_is_canonical_with_more_than_two_ranges() {
    let class = Class::new(vec![
        ClassRange { start: 'i', end: 'j' },
        ClassRange { start: 'k', end: 'l' },
        ClassRange { start: 'm', end: 'n' },
    ]);
    class.is_canonical();
}

#[test]
fn test_is_canonical_with_characters_in_ascending_order() {
    let class = Class::new(vec![
        ClassRange { start: 'o', end: 'p' },
        ClassRange { start: 'q', end: 'r' },
    ]);
    class.is_canonical();
}

