// Answer 0

#[test]
fn test_extract_class_bytes_with_valid_range() {
    struct TestExtractor {
        limit_class: usize,
    }

    impl Extractor for TestExtractor {
        fn class_over_limit_bytes(&self, cls: &ClassBytes) -> bool {
            cls.ranges().len() > self.limit_class
        }
    }

    let extractor = TestExtractor { limit_class: 5 };
    
    let range1 = ClassBytesRange::new(1, 3);
    let range2 = ClassBytesRange::new(4, 5);
    
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    
    let seq = extractor.extract_class_bytes(&class_bytes);
}

#[test]
fn test_extract_class_bytes_with_single_range() {
    struct TestExtractor {
        limit_class: usize,
    }

    impl Extractor for TestExtractor {
        fn class_over_limit_bytes(&self, cls: &ClassBytes) -> bool {
            cls.ranges().len() > self.limit_class
        }
    }

    let extractor = TestExtractor { limit_class: 2 };
    
    let range = ClassBytesRange::new(100, 100);
    
    let class_bytes = ClassBytes::new(vec![range]);
    
    let seq = extractor.extract_class_bytes(&class_bytes);
}

#[test]
fn test_extract_class_bytes_with_empty_class() {
    struct TestExtractor {
        limit_class: usize,
    }

    impl Extractor for TestExtractor {
        fn class_over_limit_bytes(&self, cls: &ClassBytes) -> bool {
            cls.ranges().len() > self.limit_class
        }
    }

    let extractor = TestExtractor { limit_class: 2 };
    
    let class_bytes = ClassBytes::empty();
    
    let seq = extractor.extract_class_bytes(&class_bytes);
}

#[test]
fn test_extract_class_bytes_with_full_range() {
    struct TestExtractor {
        limit_class: usize,
    }

    impl Extractor for TestExtractor {
        fn class_over_limit_bytes(&self, cls: &ClassBytes) -> bool {
            cls.ranges().len() > self.limit_class
        }
    }

    let extractor = TestExtractor { limit_class: 3 };
    
    let range = ClassBytesRange::new(0, 255);
    
    let class_bytes = ClassBytes::new(vec![range]);
    
    let seq = extractor.extract_class_bytes(&class_bytes);
}

