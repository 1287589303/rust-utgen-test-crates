// Answer 0

#[test]
fn test_which_captures_all() {
    let config = Config::new().which_captures(WhichCaptures::All);
}

#[test]
fn test_which_captures_implicit() {
    let config = Config::new().which_captures(WhichCaptures::Implicit);
}

#[test]
fn test_which_captures_none() {
    let config = Config::new().which_captures(WhichCaptures::None);
}

