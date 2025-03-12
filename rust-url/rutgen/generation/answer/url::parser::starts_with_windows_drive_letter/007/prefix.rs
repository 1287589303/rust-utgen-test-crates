// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_valid() {
    let input = "A|/";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_valid_alphabet() {
    let input = "Z|#";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_valid_question_mark() {
    let input = "M|?";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_valid_backslash() {
    let input = "K|\\";
    starts_with_windows_drive_letter(input);
}

