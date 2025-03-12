// Answer 0

#[test]
fn test_replacen_no_expansion_peek_none_with_limit_zero() {
    let re = Regex::new(r"\d+").unwrap();
    let hay = "abc 123 def 456 ghi";
    let limit = 0;
    let rep = "NUMBER";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_no_expansion_peek_some_with_limit_zero() {
    let re = Regex::new(r"\s+").unwrap();
    let hay = "hello   world";
    let limit = 0;
    let rep = " ";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_no_expansion_and_capture_matches_with_limit_zero() {
    let re = Regex::new(r"(\w+)<->(\w+)").unwrap();
    let hay = "foo<->bar baz<->qux";
    let limit = 0;
    let rep = "REPLACED";
    let result = re.replacen(hay, limit, rep);
}

