// Answer 0

#[test]
fn test_reversed_end_crlf() {
    let look = Look::EndCRLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_start_crlf() {
    let look = Look::StartCRLF;
    let result = look.reversed();
}

#[test]
fn test_reversed_end() {
    let look = Look::End;
    let result = look.reversed();
}

#[test]
fn test_reversed_start() {
    let look = Look::Start;
    let result = look.reversed();
}

