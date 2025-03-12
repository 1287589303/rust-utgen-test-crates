// Answer 0

#[test]
fn test_union_with_contiguous_ranges_1() {
    let range1 = ClassRange { start: 'a', end: 'c' };
    let range2 = ClassRange { start: 'b', end: 'd' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_contiguous_ranges_2() {
    let range1 = ClassRange { start: '1', end: '3' };
    let range2 = ClassRange { start: '3', end: '5' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_same_start_end() {
    let range1 = ClassRange { start: 'x', end: 'x' };
    let range2 = ClassRange { start: 'x', end: 'y' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_adjacent_chars() {
    let range1 = ClassRange { start: 'y', end: 'y' };
    let range2 = ClassRange { start: 'z', end: 'z' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_completely_overlapping_ranges() {
    let range1 = ClassRange { start: 'a', end: 'e' };
    let range2 = ClassRange { start: 'b', end: 'd' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_minimum_boundary_case() {
    let range1 = ClassRange { start: 'a', end: 'a' };
    let range2 = ClassRange { start: 'a', end: 'a' };
    let result = range1.union(&range2);
}

#[test]
fn test_union_with_maximum_boundary_case() {
    let range1 = ClassRange { start: 'A', end: 'Z' };
    let range2 = ClassRange { start: 'Z', end: 'Z' };
    let result = range1.union(&range2);
}

