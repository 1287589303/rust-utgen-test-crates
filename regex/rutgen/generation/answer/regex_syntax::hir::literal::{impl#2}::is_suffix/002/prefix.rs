// Answer 0

#[test]
fn test_is_suffix_with_suffix_variant() {
    let kind = ExtractKind::Suffix;
    let result = kind.is_suffix();
}

#[test]
fn test_is_suffix_with_prefix_variant() {
    let kind = ExtractKind::Prefix;
    let result = kind.is_suffix();
}

