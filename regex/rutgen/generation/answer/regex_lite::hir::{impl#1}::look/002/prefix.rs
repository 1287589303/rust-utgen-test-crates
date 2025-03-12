// Answer 0

#[test]
fn test_look_start() {
    let look = Look::Start;
    let result = Hir::look(look);
}

#[test]
fn test_look_end() {
    let look = Look::End;
    let result = Hir::look(look);
}

#[test]
fn test_look_start_lf() {
    let look = Look::StartLF;
    let result = Hir::look(look);
}

#[test]
fn test_look_end_lf() {
    let look = Look::EndLF;
    let result = Hir::look(look);
}

#[test]
fn test_look_start_crlf() {
    let look = Look::StartCRLF;
    let result = Hir::look(look);
}

#[test]
fn test_look_end_crlf() {
    let look = Look::EndCRLF;
    let result = Hir::look(look);
}

#[test]
fn test_look_word() {
    let look = Look::Word;
    let result = Hir::look(look);
}

#[test]
fn test_look_word_negate() {
    let look = Look::WordNegate;
    let result = Hir::look(look);
}

#[test]
fn test_look_word_start() {
    let look = Look::WordStart;
    let result = Hir::look(look);
}

#[test]
fn test_look_word_end() {
    let look = Look::WordEnd;
    let result = Hir::look(look);
}

#[test]
fn test_look_word_start_half() {
    let look = Look::WordStartHalf;
    let result = Hir::look(look);
}

#[test]
fn test_look_word_end_half() {
    let look = Look::WordEndHalf;
    let result = Hir::look(look);
}

