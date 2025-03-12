// Answer 0

#[test]
#[should_panic]
fn test_overlaps_start_greater_than_end_1() {
    let table: &[(char, &'static [char])] = &[];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start = '\u{FF00}';
    let end = '\u{00FF}';
    case_folder.overlaps(start, end);
}

#[test]
#[should_panic]
fn test_overlaps_start_greater_than_end_2() {
    let table: &[(char, &'static [char])] = &[];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start = '\u{007F}';
    let end = '\u{0000}';
    case_folder.overlaps(start, end);
}

#[test]
#[should_panic]
fn test_overlaps_start_greater_than_end_3() {
    let table: &[(char, &'static [char])] = &[];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start = '\u{1234}';
    let end = '\u{1233}';
    case_folder.overlaps(start, end);
}

#[test]
#[should_panic]
fn test_overlaps_start_greater_than_end_4() {
    let table: &[(char, &'static [char])] = &[];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start = '\u{2E00}';
    let end = '\u{2DFF}';
    case_folder.overlaps(start, end);
}

#[test]
#[should_panic]
fn test_overlaps_start_greater_than_end_5() {
    let table: &[(char, &'static [char])] = &[];
    let mut case_folder = SimpleCaseFolder { table, last: None, next: 0 };
    let start = '\u{FFFF}';
    let end = '\u{FFFE}';
    case_folder.overlaps(start, end);
}

