// Answer 0

#[test]
fn test_empty_look_set_debug() {
    let look_set = LookSet::empty();
    let mut output = core::fmt::Formatter::new();
    let _ = look_set.fmt(&mut output);
}

#[test]
fn test_singleton_look_set_debug() {
    let look_set = LookSet::singleton(Look::Start);
    let mut output = core::fmt::Formatter::new();
    let _ = look_set.fmt(&mut output);
}

#[test]
fn test_full_look_set_debug() {
    let look_set = LookSet::full();
    let mut output = core::fmt::Formatter::new();
    let _ = look_set.fmt(&mut output);
}

