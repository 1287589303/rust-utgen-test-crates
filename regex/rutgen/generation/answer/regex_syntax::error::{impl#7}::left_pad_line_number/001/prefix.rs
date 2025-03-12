// Answer 0

#[test]
fn test_left_pad_line_number_zero() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(0);
}

#[test]
fn test_left_pad_line_number_one() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(1);
}

#[test]
fn test_left_pad_line_number_two() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(2);
}

#[test]
fn test_left_pad_line_number_boundary() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 5,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(3);
}

#[test]
fn test_left_pad_line_number_equals_width() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(3);
}

#[test]
#[should_panic]
fn test_left_pad_line_number_out_of_bounds_low() {
    let spans = Spans {
        pattern: "a\nb\nc",
        line_number_width: 3,
        by_line: vec![vec![], vec![], vec![]],
        multi_line: vec![],
    };
    let result = spans.left_pad_line_number(4);
}

