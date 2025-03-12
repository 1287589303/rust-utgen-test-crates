// Answer 0

#[test]
fn test_notate_line_empty_spans_zero_index() {
    let pattern = "abc\ndef\n";
    let by_line = vec![vec![], vec![], vec![]]; // All lines are empty
    let multi_line = vec![];
    
    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate_line(0);
    assert!(result.is_none());
}

#[test]
fn test_notate_line_empty_spans_middle_index() {
    let pattern = "abc\ndef\nghi\n";
    let by_line = vec![vec![], vec![], vec![]]; // All lines are empty
    let multi_line = vec![];
    
    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate_line(1);
    assert!(result.is_none());
}

#[test]
fn test_notate_line_empty_spans_last_index() {
    let pattern = "abc\nxyz\n";
    let by_line = vec![vec![], vec![]]; // All lines are empty
    let multi_line = vec![];
    
    let spans = Spans {
        pattern,
        line_number_width: 0,
        by_line,
        multi_line,
    };

    let result = spans.notate_line(1);
    assert!(result.is_none());
}

