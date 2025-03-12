fn path_starts_with_windows_drive_letter(s: &str) -> bool {
    if let Some(c) = s.as_bytes().first() {
        matches!(c, b'/' | b'\\' | b'?' | b'#') && starts_with_windows_drive_letter(&s[1..])
    } else {
        false
    }
}