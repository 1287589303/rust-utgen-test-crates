// Answer 0

#[test]
fn test_anchored_yes() {
    let anchor = Anchored::Yes;
    anchor.is_anchored();
}

#[test]
fn test_anchored_pattern() {
    let pattern_id = PatternID::default(); // Assuming default initializes a valid PatternID
    let anchor = Anchored::Pattern(pattern_id);
    anchor.is_anchored();
}

