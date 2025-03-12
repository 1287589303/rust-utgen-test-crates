// Answer 0

#[test]
fn test_ascii_tab_or_new_line_with_tab() {
    let ch = '\t';
    ascii_tab_or_new_line(ch);
}

#[test]
fn test_ascii_tab_or_new_line_with_newline() {
    let ch = '\n';
    ascii_tab_or_new_line(ch);
}

#[test]
fn test_ascii_tab_or_new_line_with_carriage_return() {
    let ch = '\r';
    ascii_tab_or_new_line(ch);
}

