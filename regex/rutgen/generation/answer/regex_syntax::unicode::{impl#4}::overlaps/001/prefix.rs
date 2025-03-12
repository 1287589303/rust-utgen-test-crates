// Answer 0

#[test]
fn test_overlaps_same_character() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start_end = 'a';
    case_folder.overlaps(start_end, start_end);
}

#[test]
fn test_overlaps_valid_range() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    case_folder.overlaps('a', 'b');
}

#[test]
fn test_overlaps_boundary_low() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    case_folder.overlaps('\u{0000}', '\u{0000}');
}

#[test]
fn test_overlaps_boundary_high() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    case_folder.overlaps('\u{10FFFF}', '\u{10FFFF}');
}

#[test]
fn test_overlaps_non_overlapping_range() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('b', &['B'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    case_folder.overlaps('c', 'd');
}

#[test]
fn test_overlaps_inclusive_overlapping_range() {
    let table: &'static [(char, &'static [char])] = &[('a', &['A']), ('c', &['C'])];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    case_folder.overlaps('a', 'c');
}

