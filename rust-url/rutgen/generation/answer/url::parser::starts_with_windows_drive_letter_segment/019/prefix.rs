// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_segment_false_non_alpha() {
    let input = Input {
        chars: "1:".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_false_non_alpha_pipe() {
    let input = Input {
        chars: "3|".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

