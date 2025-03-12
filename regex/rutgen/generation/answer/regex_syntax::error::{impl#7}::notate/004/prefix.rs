// Answer 0

#[test]
fn test_notate_with_empty_spans_and_zero_line_number_width() {
    let pattern = "This is a single line.";
    let line_number_width = 0;
    let by_line = vec![vec![]]; // Empty spans for the line
    let multi_line = vec![]; // No multi-line spans

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = spans.notate();
}

#[test]
fn test_notate_with_non_empty_pattern_and_zero_line_number_width() {
    let pattern = "Line one.\nLine two.";
    let line_number_width = 0;
    let by_line = vec![vec![], vec![]]; // No spans for both lines
    let multi_line = vec![]; // No multi-line spans

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = spans.notate();
}

#[test]
fn test_notate_with_one_line_and_no_spans() {
    let pattern = "Just a line.";
    let line_number_width = 0;
    let by_line = vec![vec![]]; // Empty spans for the line
    let multi_line = vec![]; // No multi-line spans

    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = spans.notate();
}

