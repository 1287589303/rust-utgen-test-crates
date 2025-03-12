// Answer 0

#[test]
fn test_get_onepass_enabled_true() {
    let config = Config::new().onepass(true);
    config.get_onepass();
}

#[test]
fn test_get_onepass_enabled_false() {
    let config = Config::new().onepass(false);
    config.get_onepass();
}

#[test]
fn test_get_onepass_not_set() {
    let config = Config::new();
    config.get_onepass();
}

#[test]
#[cfg(feature = "dfa-onepass")]
fn test_get_onepass_feature_enabled() {
    let config = Config::new().onepass(Some(true));
    config.get_onepass();
}

#[test]
#[cfg(not(feature = "dfa-onepass"))]
fn test_get_onepass_feature_disabled() {
    let config = Config::new().onepass(Some(true));
    config.get_onepass();
}

