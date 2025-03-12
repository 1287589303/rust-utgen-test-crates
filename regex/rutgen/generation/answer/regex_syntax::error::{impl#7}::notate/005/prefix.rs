// Answer 0

#[test]
fn test_notate_empty_string_with_zero_width() {
    let pattern = "";
    let line_number_width = 0;
    let by_line = vec![vec![]];
    let multi_line = vec![];
    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    spans.notate();
}

#[test]
fn test_notate_single_line_no_spans() {
    let pattern = "This is a test.";
    let line_number_width = 0;
    let by_line = vec![vec![]];
    let multi_line = vec![];
    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    spans.notate();
}

#[test]
fn test_notate_multiple_lines_no_spans() {
    let pattern = "Line one.\nLine two.\nLine three.";
    let line_number_width = 0;
    let by_line = vec![vec![], vec![], vec![]];
    let multi_line = vec![];
    let spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    spans.notate();
}

