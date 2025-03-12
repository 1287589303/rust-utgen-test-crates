// Answer 0

#[test]
fn test_char_control_character_null() {
    let result = Hir::char('\0');
}

#[test]
fn test_char_control_character_delete() {
    let result = Hir::char('\u{7F}');
}

#[test]
fn test_char_boundary_character_d7ff() {
    let result = Hir::char('\u{D7FF}');
}

#[test]
fn test_char_boundary_character_e000() {
    let result = Hir::char('\u{E000}');
}

#[test]
fn test_char_boundary_character_10ffff() {
    let result = Hir::char('\u{10FFFF}');
}

#[test]
fn test_char_arbitrary_character_a() {
    let result = Hir::char('a');
}

#[test]
fn test_char_arbitrary_character_zh() {
    let result = Hir::char('中');
}

#[test]
fn test_char_arbitrary_character_emoji() {
    let result = Hir::char('😊');
}

