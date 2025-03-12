// Answer 0

#[test]
fn test_difference_non_overlapping_ranges() {
    let mut self_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'c', end: 'd' },
    ]);
    let other_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'e', end: 'f' },
    ]);
    self_class.difference(&other_class);
}

#[test]
fn test_difference_identical_ranges() {
    let mut self_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
    let other_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
    self_class.difference(&other_class);
}

#[test]
fn test_difference_overlapping_ranges() {
    let mut self_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'd' },
    ]);
    let other_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'c' },
    ]);
    self_class.difference(&other_class);
}

#[test]
fn test_difference_empty_other() {
    let mut self_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
    let other_class = ClassUnicode::empty();
    self_class.difference(&other_class);
}

#[test]
fn test_difference_multiple_non_overlapping_ranges() {
    let mut self_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'g' },
    ]);
    let other_class = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'h', end: 'i' },
    ]);
    self_class.difference(&other_class);
}

