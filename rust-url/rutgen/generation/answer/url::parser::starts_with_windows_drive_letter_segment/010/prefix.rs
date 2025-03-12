// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_segment_case1() {
    let input_str = "C:/";
    let input = Input { chars: input_str.chars() };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case2() {
    let input_str = "D:\\";
    let input = Input { chars: input_str.chars() };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case3() {
    let input_str = "E|#";
    let input = Input { chars: input_str.chars() };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case4() {
    let input_str = "F|?";
    let input = Input { chars: input_str.chars() };
    starts_with_windows_drive_letter_segment(&input);
}

