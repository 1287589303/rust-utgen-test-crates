// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_peek() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let hay = "123 apples and 456 oranges";
    let limit = 2;
    let replacement = "REPLACED";
    let result = re.replacen(hay, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_one_match() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let hay = "There are 789 bananas";
    let limit = 1;
    let replacement = "NUMBER";
    let result = re.replacen(hay, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_multiple_matches() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let hay = "1 and 2 and 3";
    let limit = 3;
    let replacement = "X";
    let result = re.replacen(hay, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_exact_limit_matches() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let hay = "100 200 300 400";
    let limit = 3;
    let replacement = "NUM";
    let result = re.replacen(hay, limit, replacement);
}

