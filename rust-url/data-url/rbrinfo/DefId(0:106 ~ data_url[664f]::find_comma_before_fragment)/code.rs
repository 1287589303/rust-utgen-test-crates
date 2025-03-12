fn find_comma_before_fragment(after_colon: &str) -> Option<(&str, &str)> {
    for (i, byte) in after_colon.bytes().enumerate() {
        if byte == b',' {
            return Some((&after_colon[..i], &after_colon[i + 1..]));
        }
        if byte == b'#' {
            break;
        }
    }
    None
}