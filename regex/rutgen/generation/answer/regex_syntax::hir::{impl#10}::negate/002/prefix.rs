// Answer 0

#[test]
fn test_negate_unicode_class_basic() {
    let mut class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new(0x00C0, 0x00FF)]));
    class_unicode.negate();
}

#[test]
fn test_negate_unicode_class_empty() {
    let mut class_unicode = Class::Unicode(ClassUnicode::empty());
    class_unicode.negate();
}

#[test]
fn test_negate_unicode_class_multiple_ranges() {
    let mut class_unicode = Class::Unicode(ClassUnicode::new(vec![
        ClassUnicodeRange::new(0x0041, 0x005A),
        ClassUnicodeRange::new(0x0061, 0x007A),
    ]));
    class_unicode.negate();
}

#[test]
fn test_negate_unicode_class_single_range() {
    let mut class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new(0x0030, 0x0039)]));
    class_unicode.negate();
}

#[test]
fn test_negate_unicode_class_non_ascii() {
    let mut class_unicode = Class::Unicode(ClassUnicode::new(vec![ClassUnicodeRange::new(0x00E0, 0x00FF)]));
    class_unicode.negate();
}

