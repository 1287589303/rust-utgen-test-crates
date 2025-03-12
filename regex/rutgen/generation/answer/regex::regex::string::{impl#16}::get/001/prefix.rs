// Answer 0

#[test]
fn test_get_valid_capture_groups() {
    use regex::Regex;
    
    let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

    let result_0 = locs.get(0);
    let result_1 = locs.get(1);
    let result_2 = locs.get(2);
}

#[test]
fn test_get_invalid_capture_group_below_range() {
    use regex::Regex;
    
    let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

    let result_minus_1 = locs.get(usize::MAX);
}

#[test]
fn test_get_invalid_capture_group_above_range() {
    use regex::Regex;
    
    let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

    let result_3 = locs.get(3);
}

