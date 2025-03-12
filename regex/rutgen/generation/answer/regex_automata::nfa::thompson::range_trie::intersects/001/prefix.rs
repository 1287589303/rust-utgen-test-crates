// Answer 0

#[test]
fn test_intersects_case1() {
    let r1 = Utf8Range { start: 0, end: 1 }; // r1.end < r2.start
    let r2 = Utf8Range { start: 2, end: 3 }; // r2.start is greater than r1.end
    intersects(r1, r2);
}

#[test]
fn test_intersects_case2() {
    let r1 = Utf8Range { start: 5, end: 10 }; // r1.end < r2.start
    let r2 = Utf8Range { start: 11, end: 15 }; // r2.start is greater than r1.end
    intersects(r1, r2);
}

#[test]
fn test_intersects_case3() {
    let r1 = Utf8Range { start: 100, end: 200 }; // r1.end < r2.start
    let r2 = Utf8Range { start: 201, end: 300 }; // r2.start is greater than r1.end
    intersects(r1, r2);
}

#[test]
fn test_intersects_case4() {
    let r1 = Utf8Range { start: 20, end: 30 }; // r1.end < r2.start
    let r2 = Utf8Range { start: 31, end: 40 }; // r2.start is greater than r1.end
    intersects(r1, r2);
}

#[test]
fn test_intersects_case5() {
    let r1 = Utf8Range { start: 0, end: 5 }; // r1.end < r2.start
    let r2 = Utf8Range { start: 6, end: 10 }; // r2.start is greater than r1.end
    intersects(r1, r2);
}

