// Answer 0

#[test]
fn test_set_remove_single() {
    let mut lookset = LookSet { bits: Look::Start as u32 | Look::End as u32 };
    let look = Look::Start;
    lookset.set_remove(look);
}

#[test]
fn test_set_remove_multiple() {
    let mut lookset = LookSet { bits: Look::Start as u32 | Look::End as u32 | Look::WordAscii as u32 };
    let look = Look::End;
    lookset.set_remove(look);
}

#[test]
fn test_set_remove_none() {
    let mut lookset = LookSet { bits: Look::Start as u32 };
    let look = Look::End;
    lookset.set_remove(look);
}

#[test]
fn test_set_remove_all() {
    let mut lookset = LookSet { bits: Look::Start as u32 | Look::End as u32 | Look::WordAscii as u32 | Look::WordUnicode as u32 };
    let look = Look::Start;
    lookset.set_remove(look);
    lookset.set_remove(Look::WordAscii);
    lookset.set_remove(Look::WordUnicode);
}

#[test]
fn test_set_remove_upper_bound() {
    let mut lookset = LookSet { bits: u32::MAX };
    let look = Look::WordEndHalfUnicode;
    lookset.set_remove(look);
}

#[test]
fn test_set_remove_lower_bound() {
    let mut lookset = LookSet { bits: 0 };
    let look = Look::Start;
    lookset.set_remove(look);
}

