fn ascii_tab_or_new_line(ch: char) -> bool {
    matches!(ch, ascii_tab_or_new_line_pattern!())
}