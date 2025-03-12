// Answer 0

#[test]
fn test_line_number_padding_zero() {
    let spans = Spans {
        pattern: "test",
        line_number_width: 0,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    spans.line_number_padding();
}

#[test]
fn test_line_number_padding_one() {
    let spans = Spans {
        pattern: "test",
        line_number_width: 1,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    spans.line_number_padding();
}

#[test]
fn test_line_number_padding_max() {
    let spans = Spans {
        pattern: "test",
        line_number_width: 100,
        by_line: vec![vec![]],
        multi_line: vec![],
    };
    spans.line_number_padding();
}

