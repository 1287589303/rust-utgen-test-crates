// Answer 0

#[test]
fn test_fmt_with_non_overlapping_span() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let span = Span { start, end };
    let mut output = core::fmt::Formatter::new();
    let _ = span.fmt(&mut output);
}

#[test]
fn test_fmt_with_identical_start_and_end() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span { start, end };
    let mut output = core::fmt::Formatter::new();
    let _ = span.fmt(&mut output);
}

#[test]
fn test_fmt_with_non_zero_start() {
    let start = Position { offset: 1, line: 1, column: 1 };
    let end = Position { offset: 2, line: 1, column: 3 };
    let span = Span { start, end };
    let mut output = core::fmt::Formatter::new();
    let _ = span.fmt(&mut output);
}

#[test]
fn test_fmt_with_boundary_case_max_offset() {
    let start = Position { offset: usize::MAX, line: 1, column: 1 };
    let end = Position { offset: usize::MAX - 1, line: 1, column: 2 };
    let span = Span { start, end };
    let mut output = core::fmt::Formatter::new();
    let _ = span.fmt(&mut output);
}

