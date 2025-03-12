// Answer 0

#[test]
fn test_add_look_valid_start() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(0).unwrap());
    let look = Look::Start;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_end() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(1).unwrap());
    let look = Look::End;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_startlf() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(2).unwrap());
    let look = Look::StartLF;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_endlf() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(3).unwrap());
    let look = Look::EndLF;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_wordascii() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(4).unwrap());
    let look = Look::WordAscii;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_wordunicascii() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(5).unwrap());
    let look = Look::WordUnicode;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_valid_wordendhalfascii() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(6).unwrap());
    let look = Look::WordEndHalfAscii;
    let _ = builder.add_look(next, look);
}

#[test]
fn test_add_look_max_state_id() {
    let mut builder = Builder::new();
    let next = StateID(SmallIndex::new(u32::MAX as usize).unwrap());
    let look = Look::End;
    let _ = builder.add_look(next, look);
}

