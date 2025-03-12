// Answer 0

#[test]
fn test_union_non_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'd', end: 'e' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'f', end: 'g' },
        ClassUnicodeRange { start: 'i', end: 'j' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'h' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_with_empty_range() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class_b = ClassUnicode::empty();
    class_a.union(&class_b);
}

#[test]
fn test_union_empty_with_non_empty() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'x', end: 'z' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_with_identical_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_with_disjoint_characters() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'a' },
        ClassUnicodeRange { start: 'b', end: 'b' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'c', end: 'c' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_with_invalid_character_range() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'z', end: 'a' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_large_number_of_ranges() {
    let mut class_a = ClassUnicode::new((1..=50).map(|i| ClassUnicodeRange { start: char::from('a' as u8 + i - 1), end: char::from('a' as u8 + i - 1) }));
    let class_b = ClassUnicode::new((51..=100).map(|i| ClassUnicodeRange { start: char::from('a' as u8 + i - 1), end: char::from('a' as u8 + i - 1) }));
    class_a.union(&class_b);
}

