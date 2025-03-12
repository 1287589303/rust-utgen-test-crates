// Answer 0

#[test]
fn test_split_basic_case() {
    let re = Regex::new(r" ").unwrap();
    let hay = "Mary had a little lamb";
    let result = re.split(hay);
}

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"X").unwrap();
    let hay = "";
    let result = re.split(hay);
}

#[test]
fn test_split_single_separator() {
    let re = Regex::new(r"X").unwrap();
    let hay = "lionXXtigerXleopard";
    let result = re.split(hay);
}

#[test]
fn test_split_multiple_separators() {
    let re = Regex::new(r"::").unwrap();
    let hay = "lion::tiger::leopard";
    let result = re.split(hay);
}

#[test]
fn test_split_contiguous_matches() {
    let re = Regex::new(r"X").unwrap();
    let hay = "XXXXaXXbXc";
    let result = re.split(hay);
}

#[test]
fn test_split_separators_at_ends() {
    let re = Regex::new(r"0").unwrap();
    let hay = "010";
    let result = re.split(hay);
}

#[test]
fn test_split_empty_regex() {
    let re = Regex::new(r"").unwrap();
    let hay = "rust";
    let result = re.split(hay);
}

#[test]
fn test_split_empty_string_utf8() {
    let re = Regex::new(r"").unwrap();
    let hay = "☃";
    let result = re.split(hay);
}

#[test]
fn test_split_with_contiguous_spaces() {
    let re = Regex::new(r" ").unwrap();
    let hay = "    a  b c";
    let result = re.split(hay);
}

#[test]
fn test_split_with_advanced_space_matching() {
    let re = Regex::new(r" +").unwrap();
    let hay = "    a  b c";
    let result = re.split(hay);
}

