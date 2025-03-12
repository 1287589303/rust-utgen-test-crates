// Answer 0

#[test]
fn test_replacen_empty_haystack_limit_zero() {
    let re = Regex::new(r"abc").unwrap();
    let hay = "";
    let limit = 0;
    let rep = "replacement";
    re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_empty_haystack_limit_one() {
    let re = Regex::new(r"abc").unwrap();
    let hay = "";
    let limit = 1;
    let rep = "replacement";
    re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_empty_haystack_limit_two() {
    let re = Regex::new(r"abc").unwrap();
    let hay = "";
    let limit = 2;
    let rep = "replacement";
    re.replacen(hay, limit, rep);
}

