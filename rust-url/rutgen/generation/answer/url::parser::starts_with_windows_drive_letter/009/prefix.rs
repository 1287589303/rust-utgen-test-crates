// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_boundary_case_colon() {
    let input = "C:";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_boundary_case_pipe() {
    let input = "C|";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_char_hash() {
    let input = "C:#";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_char_backslash() {
    let input = "C:\\";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_char_slash() {
    let input = "C/";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_invalid_char_question() {
    let input = "C?";
    starts_with_windows_drive_letter(input);
}

