// Answer 0

#[test]
fn test_is_intersection_empty_adjacent_ranges() {
    let range1 = ClassRange { start: 'a', end: 'a' };
    let range2 = ClassRange { start: 'b', end: 'b' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_overlapping_ranges() {
    let range1 = ClassRange { start: 'a', end: 'c' };
    let range2 = ClassRange { start: 'b', end: 'd' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_completely_separated_ranges() {
    let range1 = ClassRange { start: 'a', end: 'b' };
    let range2 = ClassRange { start: 'c', end: 'd' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_identical_ranges() {
    let range1 = ClassRange { start: 'a', end: 'b' };
    let range2 = ClassRange { start: 'a', end: 'b' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_single_character_ranges() {
    let range1 = ClassRange { start: 'x', end: 'x' };
    let range2 = ClassRange { start: 'y', end: 'y' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_edge_case_equal_starts() {
    let range1 = ClassRange { start: 'a', end: 'a' };
    let range2 = ClassRange { start: 'a', end: 'b' };
    range1.is_intersection_empty(&range2);
}

#[test]
fn test_is_intersection_empty_edge_case_equal_ends() {
    let range1 = ClassRange { start: 'b', end: 'b' };
    let range2 = ClassRange { start: 'a', end: 'b' };
    range1.is_intersection_empty(&range2);
}

