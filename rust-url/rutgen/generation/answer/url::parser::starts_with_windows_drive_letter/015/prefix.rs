// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_short_length() {
    let input = "A"; // Length < 2
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_non_alpha_start() {
    let input = "1:"; // Non-alphabetic start
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_second_char() {
    let input = "AB"; // s[1] is not ':' or '|'
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_third_char() {
    let input = "A:"; // Length == 2, no third character
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_third_char_value() {
    let input = "A|$"; // s[2] is not '/', '\\', '?', or '#'
    starts_with_windows_drive_letter(input);
}

