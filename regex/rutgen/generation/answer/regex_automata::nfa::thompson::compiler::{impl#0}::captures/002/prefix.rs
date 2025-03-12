// Answer 0

#[test]
fn test_captures_false() {
    let config = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(None)
        .shrink(true)
        .which_captures(WhichCaptures::None)
        .look_matcher(LookMatcher {
            lineterm: DebugByte::default(),
        });

    let _ = config.captures(false);
}

#[test]
fn test_captures_false_boundaries() {
    let config_zero = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(None)
        .shrink(true)
        .which_captures(WhichCaptures::None)
        .look_matcher(LookMatcher {
            lineterm: DebugByte::default(),
        });

    let _ = config_zero.captures(false);

    let config_max = Config::new()
        .utf8(true)
        .reverse(false)
        .nfa_size_limit(Some(usize::MAX))
        .shrink(true)
        .which_captures(WhichCaptures::None)
        .look_matcher(LookMatcher {
            lineterm: DebugByte::default(),
        });

    let _ = config_max.captures(false);
}

