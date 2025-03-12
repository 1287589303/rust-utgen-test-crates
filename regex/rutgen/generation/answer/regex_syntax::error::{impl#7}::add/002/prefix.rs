// Answer 0

#[test]
fn test_add_multi_line_span() {
    struct TestFormatter<'a> {
        _marker: &'a (),
    }
    
    let pattern = "line 1\nline 2\nline 3";
    let line_number_width = 0;
    let by_line = vec![Vec::new(); 3];
    let multi_line = Vec::new();
    
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    
    let start = Position { line: 1, column: 0 }; // starting at line 1
    let end = Position { line: 2, column: 5 }; // ending at line 2
    let span = Span::new(start, end);
    spans.add(span);
}

#[test]
fn test_add_another_multi_line_span() {
    struct TestFormatter<'a> {
        _marker: &'a (),
    }
    
    let pattern = "line A\nline B\nline C";
    let line_number_width = 0;
    let by_line = vec![Vec::new(); 3];
    let multi_line = Vec::new();
    
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    
    let start = Position { line: 2, column: 3 }; // starting at line 2
    let end = Position { line: 3, column: 1 }; // ending at line 3
    let span = Span::new(start, end);
    spans.add(span);
}

#[test]
fn test_add_large_multi_line_span() {
    struct TestFormatter<'a> {
        _marker: &'a (),
    }
    
    let pattern = "first line\nsecond line\nthird line\nfourth line";
    let line_number_width = 0;
    let by_line = vec![Vec::new(); 4];
    let multi_line = Vec::new();
    
    let mut spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };
    
    let start = Position { line: 1, column: 0 }; // starting at line 1
    let end = Position { line: 3, column: 0 }; // ending at line 3
    let span = Span::new(start, end);
    spans.add(span);
}

