// Answer 0

#[test]
fn test_mapping_with_equal_last_and_c() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('a'),
        next: 0,
    };
    let result = case_folder.mapping('a');
}

#[test]
fn test_mapping_with_last_equals_c_and_no_folding() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('b'),
        next: 1,
    };
    let result = case_folder.mapping('b');
}

#[test]
fn test_mapping_with_last_equals_c_and_beyond_table() {
    let table: &'static [(char, &'static [char])] = &[('x', &['X']), ('y', &['Y'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('y'),
        next: 1,
    };
    let result = case_folder.mapping('y');
}

