// Answer 0

#[test]
fn test_splitn_example_case_1() {
    let re = Regex::new(r"\W+").unwrap();
    let hay = "Hey! How are you?";
    let _result = re.splitn(hay, 3);
}

#[test]
fn test_splitn_example_case_2() {
    let re = Regex::new(r" ").unwrap();
    let hay = "Mary had a little lamb";
    let _result = re.splitn(hay, 3);
}

#[test]
fn test_splitn_empty_string() {
    let re = Regex::new(r"X").unwrap();
    let hay = "";
    let _result = re.splitn(hay, 3);
}

#[test]
fn test_splitn_multiple_delimiters() {
    let re = Regex::new(r"X").unwrap();
    let hay = "lionXXtigerXleopard";
    let _result = re.splitn(hay, 3);
}

#[test]
fn test_splitn_double_colon_delimiter() {
    let re = Regex::new(r"::").unwrap();
    let hay = "lion::tiger::leopard";
    let _result = re.splitn(hay, 2);
}

#[test]
fn test_splitn_single_delimiter_limit_1() {
    let re = Regex::new(r"X").unwrap();
    let hay = "abcXdef";
    let _result = re.splitn(hay, 1);
}

#[test]
fn test_splitn_no_delimiter() {
    let re = Regex::new(r"X").unwrap();
    let hay = "abcdef";
    let _result = re.splitn(hay, 2);
}

#[test]
fn test_splitn_empty_result() {
    let re = Regex::new(r"X").unwrap();
    let hay = "abcXdef";
    let _result = re.splitn(hay, 0);
}

