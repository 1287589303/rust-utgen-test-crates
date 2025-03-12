// Answer 0

#[test]
fn test_enforce_literal_len_prefix_limit_zero() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(0);
    let mut seq = Seq::new(vec![b"abc", b"def"]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_prefix_limit_one() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(1);
    let mut seq = Seq::new(vec![b"abc", b"def"]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_prefix_limit_exact_size() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(3);
    let mut seq = Seq::new(vec![b"abc", b"def"]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_prefix_limit_exceeding_size() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(2);
    let mut seq = Seq::new(vec![b"abcdef", b"ghijk"]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_prefix_limit_with_empty_literal() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(1);
    let mut seq = Seq::new(vec![b"", b"abc"]);
    extractor.enforce_literal_len(&mut seq);
}

#[test]
fn test_enforce_literal_len_prefix_none_literals() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_literal_len(5);
    let mut seq = Seq::empty();
    extractor.enforce_literal_len(&mut seq);
}

