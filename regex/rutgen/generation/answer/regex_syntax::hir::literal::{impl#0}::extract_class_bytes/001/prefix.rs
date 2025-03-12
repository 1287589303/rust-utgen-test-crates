// Answer 0

#[test]
fn test_extract_class_bytes_over_limit() {
    struct DummyHir;

    let mut extractor = Extractor::new();
    extractor.limit_class(100); // Set limit to 100

    let ranges = vec![
        ClassBytesRange::new(0, 150), // This range exceeds the limit
        ClassBytesRange::new(200, 255), // This also exceeds the limit
    ];
    let class_bytes = ClassBytes::new(ranges);

    let result = extractor.extract_class_bytes(&class_bytes);
}

#[test]
fn test_extract_class_bytes_exact_limit() {
    struct DummyHir;

    let mut extractor = Extractor::new();
    extractor.limit_class(255); // Set limit to 255

    let ranges = vec![
        ClassBytesRange::new(0, 255), // This range is at the limit
    ];
    let class_bytes = ClassBytes::new(ranges);

    let result = extractor.extract_class_bytes(&class_bytes);
}

#[test]
fn test_extract_class_bytes_high_limit() {
    struct DummyHir;

    let mut extractor = Extractor::new();
    extractor.limit_class(1000); // Set limit to 1000

    let ranges = vec![
        ClassBytesRange::new(0, 255), // This range is under the high limit
    ];
    let class_bytes = ClassBytes::new(ranges);

    let result = extractor.extract_class_bytes(&class_bytes);
}

