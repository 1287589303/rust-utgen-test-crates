// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_case1() {
    let input = "C:";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case2() {
    let input = "D|";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case3() {
    let input = "Z:/";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case4() {
    let input = "A\\";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case5() {
    let input = "E?";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case6() {
    let input = "F#";
    starts_with_windows_drive_letter(input);
}

#[test]
fn test_starts_with_windows_drive_letter_case7() {
    let input = "X|";
    starts_with_windows_drive_letter(input);
}

