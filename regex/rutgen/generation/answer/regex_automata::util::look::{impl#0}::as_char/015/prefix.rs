// Answer 0

#[test]
fn test_look_as_char_endlf() {
    let look = Look::EndLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_start() {
    let look = Look::Start;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_end() {
    let look = Look::End;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_startlf() {
    let look = Look::StartLF;
    let _ = look.as_char();
}

#[test]
fn test_look_as_char_startcrlf() {
    let look = Look::StartCRLF;
    let _ = look.as_char();
}

