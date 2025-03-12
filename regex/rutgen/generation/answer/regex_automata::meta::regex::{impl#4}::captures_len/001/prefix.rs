// Answer 0

#[test]
fn test_single_capture_groups() {
    let pattern_len = |pattern| {
        Regex::new(pattern).map(|re| re.captures_len())
    };

    let _ = pattern_len("a");
    let _ = pattern_len("(a)");
    let _ = pattern_len("(a)|(b)");
    let _ = pattern_len("(a)(b)|(c)(d)");
    let _ = pattern_len("(a)|b");
    let _ = pattern_len("a|(b)");
    let _ = pattern_len("(b)*");
    let _ = pattern_len("(b)+");
}

#[test]
fn test_multiple_capture_groups() {
    let patterns_len = |patterns| {
        Regex::new_many(patterns).map(|re| re.captures_len())
    };

    let _ = patterns_len(&["a", "b"]);
    let _ = patterns_len(&["(a)", "(b)"]);
    let _ = patterns_len(&["(a)|(b)", "(c)|(d)"]);
    let _ = patterns_len(&["(a)(b)|(c)(d)", "(x)(y)"]);
    let _ = patterns_len(&["(a)", "b"]);
    let _ = patterns_len(&["a", "(b)"]);
    let _ = patterns_len(&["(a)", "(b)*"]);
    let _ = patterns_len(&["(a)+", "(b)+"]);
}

#[test]
fn test_empty_string_pattern() {
    let pattern_len = |pattern| {
        Regex::new(pattern).map(|re| re.captures_len())
    };

    let _ = pattern_len("");
}

