// Answer 0

#[test]
fn test_class_unicode_empty_ranges() {
    let class_unicode = Class::Unicode(ClassUnicode::empty());
    let _ = format!("{:?}", class_unicode);
}

#[test]
fn test_class_unicode_single_range_equal_start_end() {
    let range = ClassUnicodeRange { start: 'a', end: 'a' };
    let class_unicode = Class::Unicode(ClassUnicode::new(vec![range]));
    let _ = format!("{:?}", class_unicode);
}

#[test]
fn test_class_unicode_single_range_specific_scalar() {
    let range = ClassUnicodeRange { start: 'B', end: 'B' };
    let class_unicode = Class::Unicode(ClassUnicode::new(vec![range]));
    let _ = format!("{:?}", class_unicode);
}

