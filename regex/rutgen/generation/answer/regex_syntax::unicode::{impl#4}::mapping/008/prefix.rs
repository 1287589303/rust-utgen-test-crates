// Answer 0

#[test]
fn test_mapping_case_fold_err() {
    let table: &[(char, &'static [char])] = &[
        ('a', &['A']),
        ('b', &['B']),
        ('c', &['C']),
        ('d', &['D']),
    ];
    
    let mut folder = SimpleCaseFolder {
        table,
        last: Some('a'),
        next: 0,
    };
    
    folder.mapping('b'); // last updated to 'b'
    
    let result = folder.mapping('e'); // last updated to 'e', returns &[]
}

#[test]
fn test_mapping_case_fold_err_boundary() {
    let table: &[(char, &'static [char])] = &[
        ('a', &['A']),
        ('b', &['B']),
        ('c', &['C']),
    ];
    
    let mut folder = SimpleCaseFolder {
        table,
        last: Some('b'),
        next: 1,
    };
    
    let result = folder.mapping('d'); // last updated to 'd', returns &[]
}

#[test]
fn test_mapping_case_fold_err_with_non_consecutive() {
    let table: &[(char, &'static [char])] = &[
        ('x', &['X']),
        ('y', &['Y']),
        ('z', &['Z']),
    ];
    
    let mut folder = SimpleCaseFolder {
        table,
        last: Some('x'),
        next: 0,
    };
    
    let result = folder.mapping('z'); // last updated to 'z', returns &[]
}

