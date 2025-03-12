// Answer 0

#[test]
fn test_reversed_start() {
    let look = Look::Start;
    let _result = look.reversed();
}

#[test]
fn test_reversed_end() {
    let look = Look::End;
    let _result = look.reversed();
}

#[test]
fn test_reversed_startlf() {
    let look = Look::StartLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_endlf() {
    let look = Look::EndLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_startcrlf() {
    let look = Look::StartCRLF;
    let _result = look.reversed();
}

#[test]
fn test_reversed_endcrlf() {
    let look = Look::EndCRLF;
    let _result = look.reversed();
}

