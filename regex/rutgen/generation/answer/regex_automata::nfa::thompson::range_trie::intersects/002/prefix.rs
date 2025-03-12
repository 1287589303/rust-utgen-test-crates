// Answer 0

#[test]
fn test_intersects_boundaries() {
    let r1 = Utf8Range { start: 0, end: 5 };
    let r2 = Utf8Range { start: 5, end: 10 };
    intersects(r1, r2);
}

#[test]
fn test_intersects_same_start_end() {
    let r1 = Utf8Range { start: 3, end: 7 };
    let r2 = Utf8Range { start: 7, end: 12 };
    intersects(r1, r2);
}

#[test]
fn test_intersects_one_unit_overlap() {
    let r1 = Utf8Range { start: 1, end: 4 };
    let r2 = Utf8Range { start: 4, end: 8 };
    intersects(r1, r2);
}

#[test]
fn test_intersects_larger_range() {
    let r1 = Utf8Range { start: 10, end: 20 };
    let r2 = Utf8Range { start: 20, end: 30 };
    intersects(r1, r2);
}

