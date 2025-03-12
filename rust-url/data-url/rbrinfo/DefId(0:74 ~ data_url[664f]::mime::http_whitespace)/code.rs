fn http_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}