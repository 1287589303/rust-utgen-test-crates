// Answer 0

#[test]
fn test_fmt_non_empty_singleton() {
    let look_set = LookSet::singleton(Look::Start);
    let mut output = core::fmt::Formatter::default();
    let _ = look_set.fmt(&mut output);
}

#[test]
fn test_fmt_non_empty_multiple() {
    let mut look_set = LookSet::empty();
    look_set = look_set.insert(Look::End);
    look_set = look_set.insert(Look::WordAscii);
    let mut output = core::fmt::Formatter::default();
    let _ = look_set.fmt(&mut output);
}

#[test]
fn test_fmt_non_empty_full() {
    let look_set = LookSet::full();
    let mut output = core::fmt::Formatter::default();
    let _ = look_set.fmt(&mut output);
}

