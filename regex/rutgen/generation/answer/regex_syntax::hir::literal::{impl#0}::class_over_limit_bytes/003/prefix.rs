// Answer 0

#[test]
fn test_class_over_limit_bytes_empty_class() {
    let extractor = Extractor::new().limit_class(0);
    let class_bytes = ClassBytes::empty();
    extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_single_range_within_limit() {
    let extractor = Extractor::new().limit_class(2);
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(1, 2)]);
    extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_multiple_ranges_within_limit() {
    let extractor = Extractor::new().limit_class(5);
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange::new(1, 2),
        ClassBytesRange::new(3, 3)
    ]);
    extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_single_range_exceeding_limit() {
    let extractor = Extractor::new().limit_class(1);
    let class_bytes = ClassBytes::new(vec![ClassBytesRange::new(1, 3)]);
    extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_multiple_ranges_exceeding_limit() {
    let extractor = Extractor::new().limit_class(3);
    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange::new(1, 2),
        ClassBytesRange::new(3, 4)
    ]);
    extractor.class_over_limit_bytes(&class_bytes);
}

