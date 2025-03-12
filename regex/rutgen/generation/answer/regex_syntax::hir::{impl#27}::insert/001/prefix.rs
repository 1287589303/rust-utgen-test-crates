// Answer 0

#[test]
fn test_insert_start() {
    let initial_set = LookSet { bits: 0x0000 };
    let result_set = initial_set.insert(Look::Start);
}

#[test]
fn test_insert_end() {
    let initial_set = LookSet { bits: 0x0000 };
    let result_set = initial_set.insert(Look::End);
}

#[test]
fn test_insert_startlf() {
    let initial_set = LookSet { bits: 0x0000 };
    let result_set = initial_set.insert(Look::StartLF);
}

#[test]
fn test_insert_endlf() {
    let initial_set = LookSet { bits: 0x0000 };
    let result_set = initial_set.insert(Look::EndLF);
}

#[test]
fn test_insert_wordascii() {
    let initial_set = LookSet { bits: 0x0000 };
    let result_set = initial_set.insert(Look::WordAscii);
}

#[test]
fn test_insert_multiple() {
    let initial_set = LookSet { bits: 0x0000 };
    let intermediate_set = initial_set.insert(Look::Start);
    let result_set = intermediate_set.insert(Look::End);
}

#[test]
fn test_insert_existing() {
    let initial_set = LookSet { bits: Look::Start.as_repr() };
    let result_set = initial_set.insert(Look::Start);
}

