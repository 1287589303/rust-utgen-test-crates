// Answer 0

#[test]
fn test_class_over_limit_unicode_case_empty_class() {
    let extractor = Extractor::new().limit_class(0);
    let class_unicode = ClassUnicode::empty();
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_case_single_range_equals_limit() {
    let extractor = Extractor::new().limit_class(1);
    let class_range = ClassUnicodeRange::new('a', 'a');
    let class_unicode = ClassUnicode::new(vec![class_range]);
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_case_single_range_less_than_limit() {
    let extractor = Extractor::new().limit_class(2);
    let class_range = ClassUnicodeRange::new('a', 'a');
    let class_unicode = ClassUnicode::new(vec![class_range]);
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_case_multiple_ranges_sum_equals_limit() {
    let extractor = Extractor::new().limit_class(3);
    let class_range1 = ClassUnicodeRange::new('a', 'a'); // length = 1
    let class_range2 = ClassUnicodeRange::new('b', 'b'); // length = 1
    let class_unicode = ClassUnicode::new(vec![class_range1, class_range2]);
    extractor.class_over_limit_unicode(&class_unicode);
}

#[test]
fn test_class_over_limit_unicode_case_single_range_exceeds_limit() {
    let extractor = Extractor::new().limit_class(1);
    let class_range = ClassUnicodeRange::new('a', 'c'); // length = 3
    let class_unicode = ClassUnicode::new(vec![class_range]);
    extractor.class_over_limit_unicode(&class_unicode);
}

