fn starts_with_windows_drive_letter(s: &str) -> bool {
    s.len() >= 2
        && ascii_alpha(s.as_bytes()[0] as char)
        && matches!(s.as_bytes()[1], b':' | b'|')
        && (s.len() == 2 || matches!(s.as_bytes()[2], b'/' | b'\\' | b'?' | b'#'))
}