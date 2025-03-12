// Answer 0

#[test]
fn test_in_inclusive_range_char_start_equals_end() {
    let c = 'a';
    let start = 'a';
    let end = 'a';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_c_equals_start() {
    let c = 'a';
    let start = 'a';
    let end = 'z';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_c_equals_end() {
    let c = 'z';
    let start = 'a';
    let end = 'z';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_c_between_start_and_end() {
    let c = 'm';
    let start = 'a';
    let end = 'z';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_c_low_bounds() {
    let c = '0';
    let start = '0';
    let end = '9';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_c_high_bounds() {
    let c = '~';
    let start = '!';
    let end = '~';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_start_high_end_low() {
    let c = 'a';
    let start = 'z';
    let end = 'a';
    let _ = in_inclusive_range_char(c, start, end);
}

#[test]
fn test_in_inclusive_range_char_maximum_ascii_values() {
    let c = 'ÿ'; // ASCII limit for testing
    let start = 'ÿ'; 
    let end = 'ÿ'; 
    let _ = in_inclusive_range_char(c, start, end);
} 

#[test]
fn test_in_inclusive_range_char_edge_case_high() {
    let c = 'A';
    let start = 'A';
    let end = 'Z';
    let _ = in_inclusive_range_char(c, start, end);
} 

#[test]
fn test_in_inclusive_range_char_edge_case_low() {
    let c = 'D';
    let start = 'A';
    let end = 'E';
    let _ = in_inclusive_range_char(c, start, end);
} 

