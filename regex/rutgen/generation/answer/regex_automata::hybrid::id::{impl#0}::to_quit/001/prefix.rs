// Answer 0

#[test]
fn test_to_quit_with_valid_id() {
    let id = LazyStateID::new_unchecked(0);
    let quit_id = id.to_quit();
}

#[test]
fn test_to_quit_with_max_id() {
    let id = LazyStateID::new_unchecked(31);
    let quit_id = id.to_quit();
}

#[test]
#[should_panic]
fn test_to_quit_with_invalid_id() {
    let id = LazyStateID::new_unchecked(32);
    let quit_id = id.to_quit();
}

#[test]
fn test_to_quit_with_unknown_tag() {
    let id = LazyStateID::new_unchecked(0).to_unknown();
    let quit_id = id.to_quit();
}

#[test]
fn test_to_quit_with_dead_tag() {
    let id = LazyStateID::new_unchecked(0).to_dead();
    let quit_id = id.to_quit();
}

#[test]
fn test_to_quit_with_start_tag() {
    let id = LazyStateID::new_unchecked(0).to_start();
    let quit_id = id.to_quit();
}

#[test]
fn test_to_quit_with_match_tag() {
    let id = LazyStateID::new_unchecked(0).to_match();
    let quit_id = id.to_quit();
}

