// Answer 0

#[test]
fn test_swap_greed_true() {
    let re = RegexBuilder::new(r"a+")
        .swap_greed(true)
        .build()
        .unwrap();
}

#[test]
fn test_swap_greed_false() {
    let re = RegexBuilder::new(r"a+")
        .swap_greed(false)
        .build()
        .unwrap();
}

#[test]
fn test_swap_greed_with_empty_pattern() {
    let re = RegexBuilder::new(r"")
        .swap_greed(true)
        .build()
        .unwrap();
}

#[test]
fn test_swap_greed_with_special_characters() {
    let re = RegexBuilder::new(r"[a-z]+")
        .swap_greed(false)
        .build()
        .unwrap();
}

#[test]
fn test_swap_greed_with_complex_pattern() {
    let re = RegexBuilder::new(r"(a+|b+){1,2}")
        .swap_greed(true)
        .build()
        .unwrap();
}

