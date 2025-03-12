// Answer 0

#[test]
fn test_get_which_captures_all() {
    let config = Config::default().which_captures(WhichCaptures::All);
    config.get_which_captures();
}

#[test]
fn test_get_which_captures_implicit() {
    let config = Config::default().which_captures(WhichCaptures::Implicit);
    config.get_which_captures();
}

#[test]
fn test_get_which_captures_none() {
    let config = Config::default().which_captures(WhichCaptures::None);
    config.get_which_captures();
}

#[test]
fn test_get_which_captures_default() {
    let config = Config::default();
    config.get_which_captures();
}

