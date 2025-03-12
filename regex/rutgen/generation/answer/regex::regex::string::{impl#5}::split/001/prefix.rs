// Answer 0

#[test]
fn test_split_with_spaces_and_tabs() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let hay = "a b \t  c\td    e";
    let _ = re.split(hay);
}

#[test]
fn test_split_single_space() {
    let re = Regex::new(r" ").unwrap();
    let hay = "Mary had a little lamb";
    let _ = re.split(hay);
}

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"X").unwrap();
    let hay = "";
    let _ = re.split(hay);
}

#[test]
fn test_split_multiple_delimiters() {
    let re = Regex::new(r"X").unwrap();
    let hay = "lionXXtigerXleopard";
    let _ = re.split(hay);
}

#[test]
fn test_split_double_colon() {
    let re = Regex::new(r"::").unwrap();
    let hay = "lion::tiger::leopard";
    let _ = re.split(hay);
}

#[test]
fn test_split_multiple_contiguous_matches() {
    let re = Regex::new(r"X").unwrap();
    let hay = "XXXXaXXbXc";
    let _ = re.split(hay);
}

#[test]
fn test_split_with_slashes() {
    let re = Regex::new(r"/").unwrap();
    let hay = "(///)";
    let _ = re.split(hay);
}

#[test]
fn test_split_with_leading_and_trailing_zero() {
    let re = Regex::new(r"0").unwrap();
    let hay = "010";
    let _ = re.split(hay);
}

#[test]
fn test_split_empty_regex() {
    let re = Regex::new(r"").unwrap();
    let hay = "rust";
    let _ = re.split(hay);
}

#[test]
fn test_split_empty_regex_with_unicode() {
    let re = Regex::new(r"").unwrap();
    let hay = "☃";
    let _ = re.split(hay);
}

#[test]
fn test_split_contiguous_spaces() {
    let re = Regex::new(r" ").unwrap();
    let hay = "    a  b c";
    let _ = re.split(hay);
}

#[test]
fn test_split_contiguous_spaces_with_plus() {
    let re = Regex::new(r" +").unwrap();
    let hay = "    a  b c";
    let _ = re.split(hay);
}

