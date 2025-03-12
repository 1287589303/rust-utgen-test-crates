fn starts_with_windows_drive_letter_segment(input: &Input<'_>) -> bool {
    let mut input = input.clone();
    match (input.next(), input.next(), input.next()) {
        // its first two code points are a Windows drive letter
        // its third code point is U+002F (/), U+005C (\), U+003F (?), or U+0023 (#).
        (Some(a), Some(b), Some(c))
            if ascii_alpha(a) && matches!(b, ':' | '|') && matches!(c, '/' | '\\' | '?' | '#') =>
        {
            true
        }
        // its first two code points are a Windows drive letter
        // its length is 2
        (Some(a), Some(b), None) if ascii_alpha(a) && matches!(b, ':' | '|') => true,
        _ => false,
    }
}