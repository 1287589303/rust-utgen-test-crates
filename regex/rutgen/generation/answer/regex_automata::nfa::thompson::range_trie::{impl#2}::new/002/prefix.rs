// Answer 0

#[test]
#[should_panic]
fn test_new_with_empty_ranges() {
    let state_id = StateID::new_unchecked(2);
    let ranges: &[Utf8Range] = &[];
    NextInsert::new(state_id, ranges);
}

#[test]
#[should_panic]
fn test_new_with_five_ranges() {
    let state_id = StateID::new_unchecked(2);
    let ranges = [
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
        Utf8Range { start: 8, end: 9 },
    ];
    NextInsert::new(state_id, &ranges);
}

#[test]
fn test_new_with_one_range() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [Utf8Range { start: 1, end: 2 }];
    NextInsert::new(state_id, &ranges);
}

#[test]
fn test_new_with_two_ranges() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
    ];
    NextInsert::new(state_id, &ranges);
}

#[test]
fn test_new_with_three_ranges() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
    ];
    NextInsert::new(state_id, &ranges);
}

#[test]
fn test_new_with_four_ranges() {
    let state_id = StateID::new_unchecked(1);
    let ranges = [
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
        Utf8Range { start: 7, end: 8 },
    ];
    NextInsert::new(state_id, &ranges);
}

