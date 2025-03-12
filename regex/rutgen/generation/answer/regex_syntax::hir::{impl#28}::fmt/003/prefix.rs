// Answer 0

#[test]
fn test_fmt_non_empty_lookset_single_look() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::Start);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", lookset);
}

#[test]
fn test_fmt_non_empty_lookset_multiple_looks() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::Start);
    lookset.set_insert(Look::End);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", lookset);
}

#[test]
fn test_fmt_non_empty_lookset_full_set() {
    let lookset = LookSet::full();
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", lookset);
}

#[test]
fn test_fmt_non_empty_lookset_with_word_looks() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::WordAscii);
    lookset.set_insert(Look::WordUnicode);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", lookset);
}

#[test]
fn test_fmt_non_empty_lookset_with_boundary_looks() {
    let mut lookset = LookSet::empty();
    lookset.set_insert(Look::StartLF);
    lookset.set_insert(Look::EndLF);
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", lookset);
}

