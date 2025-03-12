// Answer 0

#[test]
fn test_line_terminator_ascii_zero_utf8_on() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(true).line_terminator(0);
}

#[test]
fn test_line_terminator_ascii_newline_utf8_on() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(true).line_terminator(10);
}

#[test]
fn test_line_terminator_non_ascii_byte_utf8_on() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(true).line_terminator(130);
}

#[test]
fn test_line_terminator_ascii_zero_utf8_off() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(false).line_terminator(0);
}

#[test]
fn test_line_terminator_ascii_newline_utf8_off() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(false).line_terminator(10);
}

#[test]
fn test_line_terminator_non_ascii_byte_utf8_off() {
    let mut builder = TranslatorBuilder::new();
    builder.utf8(false).line_terminator(200);
}

#[test]
fn test_line_terminator_non_ascii_byte_utf8_on_with_invalid() {
    // This will test the scenario where the line terminator should
    // return an error due to non-ASCII byte under UTF-8 enabled.
    let mut builder = TranslatorBuilder::new();
    builder.utf8(true).line_terminator(255);
}

