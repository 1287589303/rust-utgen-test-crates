// Answer 0

#[test]
fn test_mapping_valid_case_1() {
    let table: &[(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('a'),
        next: 0,
    };
    let result = case_folder.mapping('a');
}

#[test]
fn test_mapping_valid_case_2() {
    let table: &[(char, &'static [char])] = &[('c', &['C']), ('d', &['D'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('c'),
        next: 0,
    };
    let result = case_folder.mapping('c');
}

#[test]
fn test_mapping_valid_case_3() {
    let table: &[(char, &'static [char])] = &[('x', &['X']), ('y', &['Y']), ('z', &['Z'])];
    let mut case_folder = SimpleCaseFolder {
        table,
        last: Some('x'),
        next: 1,
    };
    let result = case_folder.mapping('y');
}

