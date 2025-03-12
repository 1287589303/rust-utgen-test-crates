// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_segment_case_1() {
    let input = Input {
        chars: "A:".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case_2() {
    let input = Input {
        chars: "B|".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case_3() {
    let input = Input {
        chars: "C:".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case_4() {
    let input = Input {
        chars: "Z:".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_case_5() {
    let input = Input {
        chars: "X|".chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

