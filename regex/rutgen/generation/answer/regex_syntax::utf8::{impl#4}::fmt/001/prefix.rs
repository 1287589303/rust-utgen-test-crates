// Answer 0

#[test]
fn test_fmt_equal_start_end_zero() {
    let range = Utf8Range { start: 0, end: 0 };
    let mut output = fmt::Formatter::new();
    let _ = range.fmt(&mut output);
}

#[test]
fn test_fmt_equal_start_end_one() {
    let range = Utf8Range { start: 1, end: 1 };
    let mut output = fmt::Formatter::new();
    let _ = range.fmt(&mut output);
}

#[test]
fn test_fmt_equal_start_end_one_hundred() {
    let range = Utf8Range { start: 100, end: 100 };
    let mut output = fmt::Formatter::new();
    let _ = range.fmt(&mut output);
}

#[test]
fn test_fmt_equal_start_end_two_hundred_fifty() {
    let range = Utf8Range { start: 250, end: 250 };
    let mut output = fmt::Formatter::new();
    let _ = range.fmt(&mut output);
}

#[test]
fn test_fmt_equal_start_end_two_hundred_fifty_five() {
    let range = Utf8Range { start: 255, end: 255 };
    let mut output = fmt::Formatter::new();
    let _ = range.fmt(&mut output);
}

