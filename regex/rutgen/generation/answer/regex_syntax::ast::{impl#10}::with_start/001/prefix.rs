// Answer 0

#[test]
fn test_with_start_case1() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(start, end);
    let new_start = Position { offset: 1, line: 2, column: 3 };
    let result = span.with_start(new_start);
}

#[test]
fn test_with_start_case2() {
    let start = Position { offset: 1, line: 1, column: 1 };
    let end = Position { offset: 2, line: 1, column: 1 };
    let span = Span::new(start, end);
    let new_start = Position { offset: 0, line: 1, column: 1 };
    let result = span.with_start(new_start);
}

#[test]
fn test_with_start_case3() {
    let start = Position { offset: 0, line: 1, column: 1 };
    let end = Position { offset: 0, line: 1, column: 1 };
    let span = Span::new(start, end);
    let new_start = Position { offset: 10, line: 2, column: 1 };
    let result = span.with_start(new_start);
}

#[test]
fn test_with_start_case4() {
    let start = Position { offset: usize::MAX, line: usize::MAX, column: usize::MAX };
    let end = Position { offset: usize::MAX - 1, line: usize::MAX, column: usize::MAX - 1 };
    let span = Span::new(start, end);
    let new_start = Position { offset: 0, line: 1, column: 1 };
    let result = span.with_start(new_start);
}

#[test]
fn test_with_start_case5() {
    let start = Position { offset: 50, line: 20, column: 5 };
    let end = Position { offset: 100, line: 20, column: 55 };
    let span = Span::new(start, end);
    let new_start = Position { offset: 30, line: 10, column: 15 };
    let result = span.with_start(new_start);
}

