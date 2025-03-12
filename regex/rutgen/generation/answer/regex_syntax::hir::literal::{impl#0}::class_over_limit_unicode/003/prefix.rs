// Answer 0

#[test]
fn test_class_over_limit_unicode_no_ranges() {
    let extractor = Extractor::new().limit_class(0);
    let class_unicode = ClassUnicode::empty();
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_one_range_length_one() {
    let extractor = Extractor::new().limit_class(0);
    let class_unicode = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'a')]);
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_multiple_ranges_equal_limit_class() {
    let extractor = Extractor::new().limit_class(2);
    let class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'),
        ClassUnicodeRange::new('b', 'b'),
    ]);
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_multiple_ranges_exceed_limit_class() {
    let extractor = Extractor::new().limit_class(1);
    let class_unicode = ClassUnicode::new(vec![
        ClassUnicodeRange::new('a', 'a'),
        ClassUnicodeRange::new('b', 'b'),
    ]);
    extractor.class_over_limit_unicode(&class_unicode);
}

