// Answer 0

#[test]
fn test_contiguous_overlapping() {
    let range1 = ClassRange { start: 'a', end: 'c' };
    let range2 = ClassRange { start: 'b', end: 'd' };
    range1.is_contiguous(&range2);
}

#[test]
fn test_contiguous_adjacent() {
    let range1 = ClassRange { start: 'e', end: 'g' };
    let range2 = ClassRange { start: 'h', end: 'h' };
    range1.is_contiguous(&range2);
}

#[test]
fn test_non_contiguous() {
    let range1 = ClassRange { start: 'i', end: 'k' };
    let range2 = ClassRange { start: 'm', end: 'p' };
    range1.is_contiguous(&range2);
}

#[test]
fn test_edge_case_same_start_end() {
    let range1 = ClassRange { start: 'x', end: 'x' };
    let range2 = ClassRange { start: 'x', end: 'x' };
    range1.is_contiguous(&range2);
}

#[test]
fn test_edge_case_contiguous_same_end() {
    let range1 = ClassRange { start: 'x', end: 'y' };
    let range2 = ClassRange { start: 'y', end: 'y' };
    range1.is_contiguous(&range2);
}

#[test]
fn test_edge_case_start_equal_end() {
    let range1 = ClassRange { start: 'a', end: 'a' };
    let range2 = ClassRange { start: 'a', end: 'b' };
    range1.is_contiguous(&range2);
}

