// Answer 0

#[test]
fn test_is_windows_drive_letter_valid_case() {
    let segment = "C:";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_valid_case_pipe() {
    let segment = "D|";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_no_path_segment() {
    let segment = "E:";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_with_slash() {
    let segment = "F:/";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_with_backslash() {
    let segment = "G:\\";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_with_question_mark() {
    let segment = "H?";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_with_hash() {
    let segment = "I#";
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_invalid_length() {
    let segment = "A"; // This will not satisfy segment.len() == 2.
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_invalid_first_character() {
    let segment = "1:"; // Invalid first character.
    is_windows_drive_letter(segment);
}

#[test]
fn test_is_windows_drive_letter_invalid_second_character() {
    let segment = "B*"; // Invalid second character.
    is_windows_drive_letter(segment);
}

