// Answer 0

#[test]
fn test_split_valid_regex_spaces_tabs() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let hay = "a b \t  c\td    e";
    let _split = re.split(hay);
}

#[test]
fn test_split_single_space() {
    let re = Regex::new(r" ").unwrap();
    let hay = "Mary had a little lamb";
    let _split = re.split(hay);
}

#[test]
fn test_split_empty_string() {
    let re = Regex::new(r"X").unwrap();
    let hay = "";
    let _split = re.split(hay);
}

#[test]
fn test_split_contiguous_x() {
    let re = Regex::new(r"X").unwrap();
    let hay = "lionXXtigerXleopard";
    let _split = re.split(hay);
}

#[test]
fn test_split_double_colons() {
    let re = Regex::new(r"::").unwrap();
    let hay = "lion::tiger::leopard";
    let _split = re.split(hay);
}

#[test]
fn test_split_multiple_xs() {
    let re = Regex::new(r"X").unwrap();
    let hay = "XXXXaXXbXc";
    let _split = re.split(hay);
}

#[test]
fn test_split_slashes() {
    let re = Regex::new(r"/").unwrap();
    let hay = "(///)";
    let _split = re.split(hay);
}

#[test]
fn test_split_zero() {
    let re = Regex::new(r"0").unwrap();
    let hay = "010";
    let _split = re.split(hay);
}

#[test]
fn test_split_rust() {
    let re = Regex::new(r"").unwrap();
    let hay = "rust";
    let _split = re.split(hay);
}

#[test]
fn test_split_unicode() {
    let re = Regex::new(r"").unwrap();
    let hay = "☃";
    let _split = re.split(hay);
}

#[test]
fn test_split_whitespace_only() {
    let re = Regex::new(r" +").unwrap();
    let hay = "    a  b c";
    let _split = re.split(hay);
}

