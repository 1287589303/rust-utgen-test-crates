// Answer 0

#[test]
fn test_get_backtrack_backtrack_none_feature_enabled() {
    let config = Config::new().backtrack(None);
    let _ = config.get_backtrack();
}

#[test]
fn test_get_backtrack_backtrack_some_true_feature_enabled() {
    let config = Config::new().backtrack(Some(true));
    let _ = config.get_backtrack();
}

#[test]
fn test_get_backtrack_backtrack_some_false_feature_enabled() {
    let config = Config::new().backtrack(Some(false));
    let _ = config.get_backtrack();
}

#[test]
#[cfg(not(feature = "nfa-backtrack"))]
fn test_get_backtrack_backtrack_none_feature_disabled() {
    let config = Config::new().backtrack(None);
    let _ = config.get_backtrack();
}

#[test]
#[cfg(not(feature = "nfa-backtrack"))]
fn test_get_backtrack_backtrack_some_true_feature_disabled() {
    let config = Config::new().backtrack(Some(true));
    let _ = config.get_backtrack();
}

#[test]
#[cfg(not(feature = "nfa-backtrack"))]
fn test_get_backtrack_backtrack_some_false_feature_disabled() {
    let config = Config::new().backtrack(Some(false));
    let _ = config.get_backtrack();
}

