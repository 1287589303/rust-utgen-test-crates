// Answer 0

#[test]
fn test_partial_cmp_equal() {
    let pos1 = Position { offset: 5, line: 10, column: 2 };
    let pos2 = Position { offset: 5, line: 20, column: 3 };
    let result = pos1.partial_cmp(&pos2);
}

#[test]
fn test_partial_cmp_less_than() {
    let pos1 = Position { offset: 3, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 1, column: 1 };
    let result = pos1.partial_cmp(&pos2);
}

#[test]
fn test_partial_cmp_greater_than() {
    let pos1 = Position { offset: 8, line: 2, column: 1 };
    let pos2 = Position { offset: 5, line: 1, column: 1 };
    let result = pos1.partial_cmp(&pos2);
}

#[test]
fn test_partial_cmp_zero_offset() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 1, column: 1 };
    let result = pos1.partial_cmp(&pos2);
}

#[test]
fn test_partial_cmp_max_offset() {
    let pos1 = Position { offset: 100, line: 100, column: 100 };
    let pos2 = Position { offset: 99, line: 99, column: 99 };
    let result = pos1.partial_cmp(&pos2);
}

