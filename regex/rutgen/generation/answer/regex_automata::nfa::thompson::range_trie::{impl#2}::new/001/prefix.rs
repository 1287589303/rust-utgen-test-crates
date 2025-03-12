// Answer 0

#[test]
fn test_new_with_one_range() {
    let state_id = ROOT;
    let ranges = &[Utf8Range { start: 0, end: 1 }];
    let _next_insert = NextInsert::new(state_id, ranges);
}

#[test]
fn test_new_with_two_ranges() {
    let state_id = ROOT;
    let ranges = &[Utf8Range { start: 0, end: 1 }, Utf8Range { start: 2, end: 3 }];
    let _next_insert = NextInsert::new(state_id, ranges);
}

#[test]
fn test_new_with_three_ranges() {
    let state_id = ROOT;
    let ranges = &[
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
    ];
    let _next_insert = NextInsert::new(state_id, ranges);
}

#[test]
fn test_new_with_four_ranges() {
    let state_id = ROOT;
    let ranges = &[
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 2, end: 3 },
        Utf8Range { start: 4, end: 5 },
        Utf8Range { start: 6, end: 7 },
    ];
    let _next_insert = NextInsert::new(state_id, ranges);
}

