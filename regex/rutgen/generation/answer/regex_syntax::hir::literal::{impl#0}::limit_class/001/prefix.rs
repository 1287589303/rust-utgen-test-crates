// Answer 0

#[test]
fn test_limit_class_zero() {
    let mut extractor = Extractor::new();
    extractor.limit_class(0);
}

#[test]
fn test_limit_class_one() {
    let mut extractor = Extractor::new();
    extractor.limit_class(1);
}

#[test]
fn test_limit_class_four() {
    let mut extractor = Extractor::new();
    extractor.limit_class(4);
}

#[test]
fn test_limit_class_ten() {
    let mut extractor = Extractor::new();
    extractor.limit_class(10);
}

#[test]
fn test_limit_class_max_usize() {
    let mut extractor = Extractor::new();
    extractor.limit_class(usize::MAX);
}

