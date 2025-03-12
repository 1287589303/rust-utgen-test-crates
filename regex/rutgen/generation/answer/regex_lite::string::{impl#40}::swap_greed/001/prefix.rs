// Answer 0

#[test]
fn test_swap_greed_true() {
    let mut builder = RegexBuilder::new("a+");
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_false() {
    let mut builder = RegexBuilder::new("a+");
    builder.swap_greed(false);
}

#[test]
fn test_swap_greed_with_pattern() {
    let mut builder = RegexBuilder::new("b+");
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_edge_case() {
    let mut builder = RegexBuilder::new("c*");
    builder.swap_greed(false);
}

