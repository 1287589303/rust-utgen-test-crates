// Answer 0

#[test]
fn test_add_one_line_span() {
    let pattern = "abc"; // non-empty string
    let line_number_width = 2; // valid width
    let mut by_line = vec![vec![]]; // at least one Vec
    let multi_line = vec![]; // empty Vec

    struct TestFormatter<'e, E>(core::marker::PhantomData<&'e E>);
    
    let start = Position { line: 1, column: 0 }; // valid Position
    let end = Position { line: 1, column: 3 }; // valid Position
    let span = Span::new(start, end); // valid ast::Span

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.add(span); // call the function under test
}

#[test]
fn test_add_another_one_line_span() {
    let pattern = "xyz"; // non-empty string
    let line_number_width = 1; // valid width
    let mut by_line = vec![vec![]]; // at least one Vec
    let multi_line = vec![]; // empty Vec

    let start = Position { line: 1, column: 0 }; // valid Position
    let end = Position { line: 1, column: 2 }; // valid Position
    let span = Span::new(start, end); // valid ast::Span

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.add(span); // call the function under test

    let second_start = Position { line: 1, column: 3 }; // valid Position
    let second_end = Position { line: 1, column: 5 }; // valid Position
    let second_span = Span::new(second_start, second_end); // valid ast::Span

    spans.add(second_span); // call the function under test
}

#[test]
fn test_add_one_line_span_edge_case() {
    let pattern = "edge case"; // non-empty string
    let line_number_width = 0; // boundary case for width
    let mut by_line = vec![vec![]]; // at least one Vec
    let multi_line = vec![]; // empty Vec

    let start = Position { line: 1, column: 0 }; // valid Position
    let end = Position { line: 1, column: 10 }; // valid Position
    let span = Span::new(start, end); // valid ast::Span

    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    spans.add(span); // call the function under test

    let additional_start = Position { line: 1, column: 0 }; // valid Position
    let additional_end = Position { line: 1, column: 5 }; // valid Position
    let additional_span = Span::new(additional_start, additional_end); // valid ast::Span

    spans.add(additional_span); // call the function under test
}

