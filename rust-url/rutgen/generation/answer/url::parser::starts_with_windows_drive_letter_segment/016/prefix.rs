// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_segment_colon() {
    let input = Input { chars: "A:".chars() };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_pipe() {
    let input = Input { chars: "B|".chars() };
    starts_with_windows_drive_letter_segment(&input);
}

