// Answer 0

#[test]
fn test_is_anchored_yes() {
    let anchor = Anchored::Yes;
    anchor.is_anchored();
}

#[test]
fn test_is_anchored_pattern_zero() {
    let pattern_id = PatternID(0);
    let anchor = Anchored::Pattern(pattern_id);
    anchor.is_anchored();
}

#[test]
fn test_is_anchored_pattern_one() {
    let pattern_id = PatternID(1);
    let anchor = Anchored::Pattern(pattern_id);
    anchor.is_anchored();
}

