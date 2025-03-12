// Answer 0

#[test]
fn test_in_inclusive_range_char_start() {
    let c = 'a';
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_end() {
    let c = 'z';
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_within_range() {
    let c = 'm';
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_below_range() {
    let c = ' ';
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_above_range() {
    let c = '{'; 
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_start_minus_one() {
    let c = '`'; 
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_end_plus_one() {
    let c = '}'; 
    let start = 'a';
    let end = 'z';
    let result = in_inclusive_range_char(c, start, end);
}

