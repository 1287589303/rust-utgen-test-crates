// Answer 0

#[test]
fn test_iter_empty() {
    let class_unicode = ClassUnicode::empty();
    let _iter = class_unicode.iter();
}

#[test]
fn test_iter_single_range() {
    let range = ClassUnicodeRange { start: 'a', end: 'c' };
    let class_unicode = ClassUnicode::new(vec![range]);
    let _iter = class_unicode.iter();
}

#[test]
fn test_iter_multiple_ranges() {
    let range1 = ClassUnicodeRange { start: 'a', end: 'b' };
    let range2 = ClassUnicodeRange { start: 'd', end: 'e' };
    let class_unicode = ClassUnicode::new(vec![range1, range2]);
    let _iter = class_unicode.iter();
}

