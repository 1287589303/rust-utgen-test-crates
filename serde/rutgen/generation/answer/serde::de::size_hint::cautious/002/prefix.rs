// Answer 0

#[test]
fn test_cautious_hint_none() {
    let result = cautious::<()>(None);
}

#[test]
fn test_cautious_hint_some_0() {
    let result = cautious::<()>(Some(0));
}

#[test]
fn test_cautious_hint_some_1() {
    let result = cautious::<()>(Some(1));
}

#[test]
fn test_cautious_hint_some_1024_mb() {
    let result = cautious::<()>(Some(1024 * 1024));
}

#[test]
fn test_cautious_hint_some_1024_mb_plus_1() {
    let result = cautious::<()>(Some(1024 * 1024 + 1));
}

