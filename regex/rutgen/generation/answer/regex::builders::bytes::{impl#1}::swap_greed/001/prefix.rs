// Answer 0

#[test]
fn test_swap_greed_true() {
    let mut builder = RegexSetBuilder::new(vec!["a+", "b?"]);
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_false() {
    let mut builder = RegexSetBuilder::new(vec!["a+", "b?"]);
    builder.swap_greed(false);
}

