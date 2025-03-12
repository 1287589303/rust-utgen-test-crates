// Answer 0

#[test]
#[should_panic]
fn test_new_with_empty_ranges() {
    let state_id = ROOT;
    let ranges: &[Utf8Range] = &[];
    NextInsert::new(state_id, ranges);
}

#[test]
#[should_panic]
fn test_new_with_exceeding_length_ranges() {
    let state_id = ROOT;
    let ranges: &[Utf8Range] = &[Utf8Range { start: 0, end: 1 }, 
                                  Utf8Range { start: 2, end: 3 },
                                  Utf8Range { start: 4, end: 5 },
                                  Utf8Range { start: 6, end: 7 },
                                  Utf8Range { start: 8, end: 9 }];
    NextInsert::new(state_id, ranges);
}

