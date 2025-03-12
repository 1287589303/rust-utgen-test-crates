// Answer 0

#[test]
fn test_set_insert_start() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::Start);
}

#[test]
fn test_set_insert_end() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::End);
}

#[test]
fn test_set_insert_startlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::StartLF);
}

#[test]
fn test_set_insert_endlf() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::EndLF);
}

#[test]
fn test_set_insert_wordascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordAscii);
}

#[test]
fn test_set_insert_wordunicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordUnicode);
}

#[test]
fn test_set_insert_wordstartascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartAscii);
}

#[test]
fn test_set_insert_wordendascii() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndAscii);
}

#[test]
fn test_set_insert_wordstartunicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordStartUnicode);
}

#[test]
fn test_set_insert_wordendunicode() {
    let mut look_set = LookSet::empty();
    look_set.set_insert(Look::WordEndUnicode);
}

