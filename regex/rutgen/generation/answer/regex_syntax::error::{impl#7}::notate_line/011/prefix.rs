// Answer 0

#[test]
fn test_notate_line_with_valid_span() {
    struct TestFormatter<'e, E> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "a(b+c)d";
    let line_number_width = 2;
    let spans = vec![
        ast::Span {
            start: Position { column: 2 },
            end: Position { column: 4 },
        },
    ];
    let by_line = vec![spans];
    let multi_line = vec![];

    let mut test_spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = test_spans.notate_line(0);
}

#[test]
fn test_notate_line_with_multiple_spans() {
    struct TestFormatter<'e, E> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "hello(world)!";
    let line_number_width = 3;
    let spans = vec![
        ast::Span {
            start: Position { column: 6 },
            end: Position { column: 11 },
        },
        ast::Span {
            start: Position { column: 1 },
            end: Position { column: 2 },
        },
    ];
    let by_line = vec![spans];
    let multi_line = vec![];

    let mut test_spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = test_spans.notate_line(0);
}

#[test]
fn test_notate_line_with_padding() {
    struct TestFormatter<'e, E> {
        _marker: std::marker::PhantomData<&'e E>,
    }

    let pattern = "example test";
    let line_number_width = 5;
    let spans = vec![
        ast::Span {
            start: Position { column: 3 },
            end: Position { column: 6 },
        },
    ];
    let by_line = vec![spans];
    let multi_line = vec![];

    let mut test_spans = Spans {
        pattern,
        line_number_width,
        by_line,
        multi_line,
    };

    let result = test_spans.notate_line(0);
}

