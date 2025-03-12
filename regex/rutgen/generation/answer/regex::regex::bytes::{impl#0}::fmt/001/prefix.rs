// Answer 0

#[test]
fn test_fmt_valid_regex1() {
    let pattern = Arc::from("^[a-z]+$");
    let regex = Regex { meta: meta::Regex::new(pattern.clone()).unwrap(), pattern };
    let mut output = String::new();
    let result = regex.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_valid_regex2() {
    let pattern = Arc::from(".*\\.txt$");
    let regex = Regex { meta: meta::Regex::new(pattern.clone()).unwrap(), pattern };
    let mut output = String::new();
    let result = regex.fmt(&mut output).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_invalid_regex1() {
    let pattern = Arc::from("[a-z");
    let regex = Regex { meta: meta::Regex::new(pattern.clone()).unwrap_err(), pattern };
    let mut output = String::new();
    let result = regex.fmt(&mut output).unwrap();
}

#[test]
#[should_panic]
fn test_fmt_invalid_regex2() {
    let pattern = Arc::from("(*))");
    let regex = Regex { meta: meta::Regex::new(pattern.clone()).unwrap_err(), pattern };
    let mut output = String::new();
    let result = regex.fmt(&mut output).unwrap();
}

