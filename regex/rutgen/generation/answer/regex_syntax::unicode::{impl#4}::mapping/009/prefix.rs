// Answer 0

#[test]
fn test_mapping_case_fold() {
    let table: &'static [(char, &'static [char])] = &[
        ('a', &['A']),
        ('b', &['B']),
        ('c', &['C']),
        ('d', &['D']),
        ('e', &['E']),
    ];

    let mut folder = SimpleCaseFolder {
        table,
        last: Some('a'),
        next: 2, // point to the third entry
    };

    let c: char = 'd'; // 'd' is the next character, should match 'd' in the table.
    let result = folder.mapping(c);
}

#[test]
fn test_mapping_case_fold_with_large_index() {
    let table: &'static [(char, &'static [char])] = &[
        ('a', &['A']),
        ('b', &['B']),
        ('c', &['C']),
        ('e', &['E']),
        ('f', &['F']),
    ];

    let mut folder = SimpleCaseFolder {
        table,
        last: Some('a'),
        next: 2, // point to the third entry
    };

    let c: char = 'f'; // 'f' is beyond 'e' and not at table index 2.
    let result = folder.mapping(c);
}

