// Answer 0

#[test]
fn test_union_non_contiguous_ranges1() {
    let range1 = ClassRange { start: 'a', end: 'c' };
    let range2 = ClassRange { start: 'e', end: 'f' };
    let _result = range1.union(&range2);
}

#[test]
fn test_union_non_contiguous_ranges2() {
    let range1 = ClassRange { start: 'x', end: 'z' };
    let range2 = ClassRange { start: 'a', end: 'b' };
    let _result = range1.union(&range2);
}

#[test]
fn test_union_non_contiguous_ranges3() {
    let range1 = ClassRange { start: '1', end: '3' };
    let range2 = ClassRange { start: '5', end: '7' };
    let _result = range1.union(&range2);
}

#[test]
fn test_union_non_contiguous_ranges4() {
    let range1 = ClassRange { start: 'A', end: 'C' };
    let range2 = ClassRange { start: 'E', end: 'F' };
    let _result = range1.union(&range2);
}

