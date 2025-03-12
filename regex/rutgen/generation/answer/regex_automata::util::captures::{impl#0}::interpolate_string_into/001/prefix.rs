// Answer 0

#[test]
fn test_interpolate_string_into_valid_match() {
    let group_info = GroupInfo::default();
    let haystack = "On 14-03-2010, I became a Tenneessee lamb.";
    let replacement = "year=$year, month=$month, day=$day";
    let mut dst = String::new();
    
    let captures = Captures::all(group_info.clone());
    captures.interpolate_string_into(haystack, replacement, &mut dst);
}

#[test]
fn test_interpolate_string_into_valid_match_second_pattern() {
    let group_info = GroupInfo::default();
    let haystack = "On 2010-03-14, I became a Tenneessee lamb.";
    let replacement = "year=$year, month=$month, day=$day";
    let mut dst = String::new();
    
    let captures = Captures::all(group_info.clone());
    captures.interpolate_string_into(haystack, replacement, &mut dst);
}

#[test]
fn test_interpolate_string_into_empty_replacement() {
    let group_info = GroupInfo::default();
    let haystack = "Sample text for interpolation.";
    let replacement = "";
    let mut dst = String::new();
    
    let captures = Captures::all(group_info.clone());
    captures.interpolate_string_into(haystack, replacement, &mut dst);
}

#[test]
fn test_interpolate_string_into_invalid_reference() {
    let group_info = GroupInfo::default();
    let haystack = "Data without matches.";
    let replacement = "Invalid reference $invalid.";
    let mut dst = String::new();
    
    let captures = Captures::all(group_info.clone());
    captures.interpolate_string_into(haystack, replacement, &mut dst);
}

#[test]
fn test_interpolate_string_into_multiple_references() {
    let group_info = GroupInfo::default();
    let haystack = "Date is 01-01-2020.";
    let replacement = "Year: $year, Month: $month, Day: $day";
    let mut dst = String::new();
    
    let captures = Captures::all(group_info.clone());
    captures.interpolate_string_into(haystack, replacement, &mut dst);
}

