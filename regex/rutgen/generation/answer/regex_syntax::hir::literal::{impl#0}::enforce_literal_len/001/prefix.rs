// Answer 0

#[test]
fn test_enforce_literal_len_suffix_with_literally_greater_than_limit() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_literal_len(5);
    let mut seq = Seq::new(vec![b"short".to_vec(), b"thisisalongliteral".to_vec()]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_suffix_with_multiple_literally_greater_than_limit() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_literal_len(3);
    let mut seq = Seq::new(vec![b"one".to_vec(), b"two".to_vec(), b"threeletters".to_vec()]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_suffix_with_exact_limit() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_literal_len(7);
    let mut seq = Seq::new(vec![b"1234567".to_vec(), b"abcdefg".to_vec()]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_suffix_with_boundaries() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_literal_len(0);
    let mut seq = Seq::new(vec![b"longliteral".to_vec(), b"anotherlongliteral".to_vec()]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_suffix_with_no_impacted_literals() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_literal_len(10);
    let mut seq = Seq::new(vec![b"short".to_vec(), b"tiny".to_vec()]);
    extractor.enforce_literal_len(&mut seq);
}

