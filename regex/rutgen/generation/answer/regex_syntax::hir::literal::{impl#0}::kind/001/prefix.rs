// Answer 0

#[test]
fn test_kind_prefix() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix);
}

#[test]
fn test_kind_suffix() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Suffix);
}

#[test]
fn test_kind_with_limits() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix)
             .limit_class(1)
             .limit_repeat(1)
             .limit_literal_len(1)
             .limit_total(1);
}

#[test]
fn test_kind_with_zero_limits() {
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Suffix)
             .limit_class(0)
             .limit_repeat(0)
             .limit_literal_len(0)
             .limit_total(0);
}

