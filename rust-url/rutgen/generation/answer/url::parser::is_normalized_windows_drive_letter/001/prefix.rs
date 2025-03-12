// Answer 0

#[test]
fn test_normalized_windows_drive_letter_valid() {
    let segment = "C:";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_valid_lowercase() {
    let segment = "c:";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_invalid_length() {
    let segment = "C";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_invalid_characters() {
    let segment = "1:";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_invalid_colon() {
    let segment = "C-";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_boundary() {
    let segment = "Z:";
    let result = is_normalized_windows_drive_letter(segment);
}

#[test]
fn test_normalized_windows_drive_letter_boundary_lowercase() {
    let segment = "z:";
    let result = is_normalized_windows_drive_letter(segment);
}

