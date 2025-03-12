// Answer 0

#[test]
fn test_cmp_equal_offsets() {
    let pos1 = Position { offset: 5, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 2, column: 2 };
    let _result = pos1.cmp(&pos2);
}

#[test]
fn test_cmp_first_less_than_second() {
    let pos1 = Position { offset: 3, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 2, column: 2 };
    let _result = pos1.cmp(&pos2);
}

#[test]
fn test_cmp_first_greater_than_second() {
    let pos1 = Position { offset: 7, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 2, column: 2 };
    let _result = pos1.cmp(&pos2);
}

#[test]
fn test_cmp_boundary_minimum() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 1, line: 1, column: 1 };
    let _result = pos1.cmp(&pos2);
}

#[test]
fn test_cmp_boundary_maximum() {
    let pos1 = Position { offset: usize::MAX, line: 1, column: 1 };
    let pos2 = Position { offset: usize::MAX - 1, line: 2, column: 2 };
    let _result = pos1.cmp(&pos2);
}

#[test]
fn test_cmp_varying_lines() {
    let pos1 = Position { offset: 4, line: 2, column: 3 };
    let pos2 = Position { offset: 4, line: 1, column: 2 };
    let _result = pos1.cmp(&pos2);
}

