// Answer 0

#[test]
fn test_swap_greed_some_true() {
    let flags = Flags { swap_greed: Some(true), ..Flags::default() };
    flags.swap_greed();
}

#[test]
fn test_swap_greed_some_false() {
    let flags = Flags { swap_greed: Some(false), ..Flags::default() };
    flags.swap_greed();
}

#[test]
fn test_swap_greed_none() {
    let flags = Flags { swap_greed: None, ..Flags::default() };
    flags.swap_greed();
}

