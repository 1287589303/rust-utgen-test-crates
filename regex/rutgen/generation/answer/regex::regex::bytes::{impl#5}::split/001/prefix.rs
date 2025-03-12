// Answer 0

#[test]
fn test_split_basic_usage() {
    let re = Regex::new(r" ").unwrap();
    let hay = b"Mary had a little lamb";
    let split = re.split(hay);
}

#[test]
fn test_split_empty_haystack() {
    let re = Regex::new(r"X").unwrap();
    let hay = b"";
    let split = re.split(hay);
}

#[test]
fn test_split_with_multiple_matches() {
    let re = Regex::new(r"X").unwrap();
    let hay = b"lionXXtigerXleopard";
    let split = re.split(hay);
}

#[test]
fn test_split_special_characters() {
    let re = Regex::new(r"::").unwrap();
    let hay = b"lion::tiger::leopard";
    let split = re.split(hay);
}

#[test]
fn test_split_contiguous_matches_empty_spans() {
    let re = Regex::new(r"X").unwrap();
    let hay = b"XXXXaXXbXc";
    let split = re.split(hay);
}

#[test]
fn test_split_separators_start_end() {
    let re = Regex::new(r"0").unwrap();
    let hay = b"010";
    let split = re.split(hay);
}

#[test]
fn test_split_empty_string_match() {
    let re = Regex::new(r"").unwrap();
    let hay = "☃".as_bytes();
    let split = re.split(hay);
}

#[test]
fn test_split_with_contiguous_spaces() {
    let re = Regex::new(r" ").unwrap();
    let hay = b"    a  b c";
    let split = re.split(hay);
}

#[test]
fn test_split_with_contiguous_space_pattern() {
    let re = Regex::new(r" +").unwrap();
    let hay = b"    a  b c";
    let split = re.split(hay);
}

