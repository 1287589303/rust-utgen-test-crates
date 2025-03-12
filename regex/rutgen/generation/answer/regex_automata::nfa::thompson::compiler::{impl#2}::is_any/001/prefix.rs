// Answer 0

#[test]
fn test_is_any_with_all() {
    let captures = WhichCaptures::All;
    captures.is_any();
}

#[test]
fn test_is_any_with_implicit() {
    let captures = WhichCaptures::Implicit;
    captures.is_any();
}

#[test]
fn test_is_any_with_none() {
    let captures = WhichCaptures::None;
    captures.is_any();
}

