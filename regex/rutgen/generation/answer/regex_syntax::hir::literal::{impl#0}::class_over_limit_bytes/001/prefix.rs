// Answer 0

#[test]
fn test_class_over_limit_bytes_exceeding_limit() {
    let mut extractor = Extractor::new();
    extractor.limit_class(5);

    let range1 = ClassBytesRange::new(0, 2); // Length 3
    let range2 = ClassBytesRange::new(3, 5); // Length 3
    // Cumulative length: 6 (exceeds limit 5)

    let mut class_bytes = ClassBytes::new(vec![range1, range2]);
    
    let result = extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_boundary_limit() {
    let mut extractor = Extractor::new();
    extractor.limit_class(5);

    let range1 = ClassBytesRange::new(0, 2); // Length 3
    let range2 = ClassBytesRange::new(3, 4); // Length 2
    // Cumulative length: 5 (equal to limit)

    let mut class_bytes = ClassBytes::new(vec![range1, range2]);
    
    let result = extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_single_exceeding_range() {
    let mut extractor = Extractor::new();
    extractor.limit_class(3);

    let range = ClassBytesRange::new(0, 5); // Length 6

    let mut class_bytes = ClassBytes::new(vec![range]);
    
    let result = extractor.class_over_limit_bytes(&class_bytes);
}

