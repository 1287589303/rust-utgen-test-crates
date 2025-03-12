// Answer 0

#[test]
fn test_ranges_empty() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [];
    let len = 0;
    let next_insert = NextInsert { state_id, ranges, len };
    let result = next_insert.ranges();
}

#[test]
fn test_ranges_one_element() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [Utf8Range::new(0, 1)];
    let len = 1;
    let next_insert = NextInsert { state_id, ranges, len };
    let result = next_insert.ranges();
}

#[test]
fn test_ranges_two_elements() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range::new(0, 1),
        Utf8Range::new(2, 3),
    ];
    let len = 2;
    let next_insert = NextInsert { state_id, ranges, len };
    let result = next_insert.ranges();
}

#[test]
fn test_ranges_three_elements() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range::new(0, 1),
        Utf8Range::new(2, 3),
        Utf8Range::new(4, 5),
    ];
    let len = 3;
    let next_insert = NextInsert { state_id, ranges, len };
    let result = next_insert.ranges();
}

#[test]
fn test_ranges_four_elements() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range::new(0, 1),
        Utf8Range::new(2, 3),
        Utf8Range::new(4, 5),
        Utf8Range::new(6, 7),
    ];
    let len = 4;
    let next_insert = NextInsert { state_id, ranges, len };
    let result = next_insert.ranges();
}

