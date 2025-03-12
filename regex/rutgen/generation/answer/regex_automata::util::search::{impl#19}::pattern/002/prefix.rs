// Answer 0

#[test]
fn test_pattern_with_valid_pattern_id() {
    let pid = PatternID(1);
    let anchored = Anchored::Pattern(pid);
    let result = anchored.pattern();
}

#[test]
fn test_pattern_with_another_valid_pattern_id() {
    let pid = PatternID(42);
    let anchored = Anchored::Pattern(pid);
    let result = anchored.pattern();
}

#[test]
fn test_pattern_with_large_pattern_id() {
    let pid = PatternID(999);
    let anchored = Anchored::Pattern(pid);
    let result = anchored.pattern();
}

