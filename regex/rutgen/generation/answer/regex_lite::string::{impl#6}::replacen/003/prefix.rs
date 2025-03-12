// Answer 0

#[test]
fn test_replacen_multiple_matches_no_expansion() {
    let re = Regex::new(r"\d+").unwrap();
    let hay = "The year 1973, the year 1975, the year 1980.";
    let limit = 2;
    let rep = "XXX";
    let new = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_spacing() {
    let re = Regex::new(r"\s+").unwrap();
    let hay = "Replace    this    with  one   space.";
    let limit = 3;
    let rep = " ";
    let new = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_special_characters() {
    let re = Regex::new(r"[!@#\\$%\\^&*()]+").unwrap();
    let hay = "Hello!!! Is this @test# working?";
    let limit = 2;
    let rep = "*";
    let new = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_exceeding_limit() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let hay = "1 22 333 4444";
    let limit = 3;
    let rep = "NUM";
    let new = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_consecutive_matches() {
    let re = Regex::new(r"([a-z]+)").unwrap();
    let hay = "hello world hello universe";
    let limit = 1;
    let rep = "greeting";
    let new = re.replacen(hay, limit, rep);
}

