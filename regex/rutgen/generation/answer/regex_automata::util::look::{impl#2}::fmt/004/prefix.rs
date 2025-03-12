// Answer 0

#[test]
fn test_fmt_non_empty_lookset_singleton() {
    let look_set = LookSet::singleton(Look::Start);
    let mut output = Vec::new();
    let result = look_set.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_non_empty_lookset_multiple() {
    let look_set = LookSet::empty()
        .insert(Look::Start)
        .insert(Look::End)
        .insert(Look::WordAscii);
    let mut output = Vec::new();
    let result = look_set.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_non_empty_lookset_full() {
    let look_set = LookSet::full();
    let mut output = Vec::new();
    let result = look_set.fmt(&mut output).unwrap();
}

