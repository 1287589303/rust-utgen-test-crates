// Answer 0

#[test]
fn test_get_anchored_no() {
    let config = Config {
        look_behind: None,
        anchored: Anchored::No,
    };
    config.get_anchored();
}

#[test]
fn test_get_anchored_yes() {
    let config = Config {
        look_behind: Some(0),
        anchored: Anchored::Yes,
    };
    config.get_anchored();
}

#[test]
fn test_get_anchored_pattern() {
    struct PatternID;
    let pattern_id = PatternID;
    
    let config = Config {
        look_behind: Some(255),
        anchored: Anchored::Pattern(pattern_id),
    };
    config.get_anchored();
}

