// Answer 0

#[test]
fn test_swap_greed_true() {
    let mut regex_builder = RegexBuilder::new(r"a+");
    let _ = regex_builder.swap_greed(true);
}

#[test]
fn test_swap_greed_false() {
    let mut regex_builder = RegexBuilder::new(r"a+");
    let _ = regex_builder.swap_greed(false);
}

#[test]
fn test_swap_greed_multiple_calls() {
    let mut regex_builder = RegexBuilder::new(r"a+");
    let _ = regex_builder.swap_greed(true).swap_greed(false).swap_greed(true);
}

#[test]
fn test_swap_greed_with_patterns() {
    let mut regex_builder = RegexBuilder::new(r"ab+c");
    let _ = regex_builder.swap_greed(true);
}

