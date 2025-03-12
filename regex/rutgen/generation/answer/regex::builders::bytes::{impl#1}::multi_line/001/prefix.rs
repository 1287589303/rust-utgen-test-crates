// Answer 0

#[test]
fn test_multi_line_true_newline() {
    let patterns = vec![r"^foo$"];
    let re_set = RegexSetBuilder::new(patterns)
        .multi_line(true)
        .build()
        .unwrap();
    let input = b"\nfoo\n";
    let _ = re_set.is_match(input);
}

#[test]
fn test_multi_line_true_crlf() {
    let patterns = vec![r"^foo$"];
    let re_set = RegexSetBuilder::new(patterns)
        .multi_line(true)
        .build()
        .unwrap();
    let input = b"\r\nfoo\r\n";
    let _ = re_set.is_match(input);
}

#[test]
fn test_multi_line_false_newline() {
    let patterns = vec![r"^foo$"];
    let re_set = RegexSetBuilder::new(patterns)
        .multi_line(false)
        .build()
        .unwrap();
    let input = b"\nfoo\n";
    let _ = re_set.is_match(input);
}

#[test]
fn test_multi_line_false_crlf() {
    let patterns = vec![r"^foo$"];
    let re_set = RegexSetBuilder::new(patterns)
        .multi_line(false)
        .build()
        .unwrap();
    let input = b"\r\nfoo\r\n";
    let _ = re_set.is_match(input);
}

#[test]
fn test_multi_line_custom_byte() {
    let patterns = vec![r"^foo$"];
    let re_set = RegexSetBuilder::new(patterns)
        .multi_line(true)
        .line_terminator(0x0A) // Custom byte for line terminator
        .build()
        .unwrap();
    let input = b"\x0Afoo\x0A";
    let _ = re_set.is_match(input);
}

