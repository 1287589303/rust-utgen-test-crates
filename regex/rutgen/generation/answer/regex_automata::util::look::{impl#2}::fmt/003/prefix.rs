// Answer 0

#[test]
fn test_look_set_singleton_start() {
    let look_set = LookSet::singleton(Look::Start);
    let mut output = String::new();
    let result = core::fmt::write(&mut output, |f| look_set.fmt(f));
    let _ = result.unwrap();
}

#[test]
fn test_look_set_all() {
    let look_set = LookSet {
        bits: Look::Start as u32 | Look::End as u32 | Look::StartLF as u32 | Look::EndLF as u32,
        ..LookSet::default()
    };
    let mut output = String::new();
    let result = core::fmt::write(&mut output, |f| look_set.fmt(f));
    let _ = result.unwrap();
}

#[test]
fn test_look_set_contains_multiple() {
    let look_set = LookSet {
        bits: Look::Start as u32 | Look::StartLF as u32 | Look::WordAscii as u32,
        ..LookSet::default()
    };
    let mut output = String::new();
    let result = core::fmt::write(&mut output, |f| look_set.fmt(f));
    let _ = result.unwrap();
}

#[test]
fn test_look_set_full() {
    let look_set = LookSet::full();
    let mut output = String::new();
    let result = core::fmt::write(&mut output, |f| look_set.fmt(f));
    let _ = result.unwrap();
}

