// Answer 0

#[test]
fn test_starts_with_windows_drive_letter_segment_valid_case_1() {
    let input_str = "C:/example";
    let input = Input {
        chars: input_str.chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_valid_case_2() {
    let input_str = "D:|example";
    let input = Input {
        chars: input_str.chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_valid_case_3() {
    let input_str = "E:\\example";
    let input = Input {
        chars: input_str.chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_valid_case_4() {
    let input_str = "F:?example";
    let input = Input {
        chars: input_str.chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

#[test]
fn test_starts_with_windows_drive_letter_segment_valid_case_5() {
    let input_str = "G:#example";
    let input = Input {
        chars: input_str.chars(),
    };
    starts_with_windows_drive_letter_segment(&input);
}

