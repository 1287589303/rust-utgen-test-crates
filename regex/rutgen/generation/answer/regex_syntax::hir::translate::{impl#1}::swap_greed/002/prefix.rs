// Answer 0

#[test]
fn test_swap_greed_false() {
    let mut builder = TranslatorBuilder::new();
    let result = builder.swap_greed(false);
}

#[test]
fn test_swap_greed_false_after_initialization() {
    let mut builder = TranslatorBuilder::new()
        .utf8(true)
        .line_terminator(b'\n')
        .case_insensitive(true);
    let result = builder.swap_greed(false);
}

#[test]
fn test_swap_greed_false_with_other_flags() {
    let mut builder = TranslatorBuilder::new()
        .multi_line(true)
        .dot_matches_new_line(false)
        .unicode(true);
    let result = builder.swap_greed(false);
}

