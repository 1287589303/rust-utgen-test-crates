// Answer 0

#[test]
fn test_class_over_limit_bytes_exact_limit() {
    struct TestHirClassBytes {
        class_bytes: hir::ClassBytes,
    }

    let mut extractor = Extractor::new();
    extractor.limit_class(5);

    let range1 = ClassBytesRange::new(0, 1); // Length 2
    let range2 = ClassBytesRange::new(2, 3); // Length 2
    let range3 = ClassBytesRange::new(4, 4); // Length 1
    let class_bytes = ClassBytes::new(vec![range1, range2, range3]);

    let result = extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_below_limit() {
    struct TestHirClassBytes {
        class_bytes: hir::ClassBytes,
    }

    let mut extractor = Extractor::new();
    extractor.limit_class(5);

    let range1 = ClassBytesRange::new(0, 0); // Length 1
    let class_bytes = ClassBytes::new(vec![range1]); // Total length is 1
    let result = extractor.class_over_limit_bytes(&class_bytes);
}

#[test]
fn test_class_over_limit_bytes_no_ranges() {
    struct TestHirClassBytes {
        class_bytes: hir::ClassBytes,
    }

    let mut extractor = Extractor::new();
    extractor.limit_class(5);
    
    let class_bytes = ClassBytes::empty(); // No ranges
    let result = extractor.class_over_limit_bytes(&class_bytes);
}

