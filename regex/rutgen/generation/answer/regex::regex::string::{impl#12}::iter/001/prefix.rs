// Answer 0

#[test]
fn test_iter_empty_string() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("").unwrap();

    let mut it = caps.iter();
    let first = it.next();
    let second = it.next();
    let third = it.next();
    let fourth = it.next();
    let fifth = it.next();

    // Call the iterator to exercise the code
    drop(first);
    drop(second);
    drop(third);
    drop(fourth);
    drop(fifth);
}

#[test]
fn test_iter_single_match() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A1B").unwrap();

    let mut it = caps.iter();
    let first = it.next();
    let second = it.next();
    let third = it.next();
    let fourth = it.next();
    let fifth = it.next();

    // Call the iterator to exercise the code
    drop(first);
    drop(second);
    drop(third);
    drop(fourth);
    drop(fifth);
}

#[test]
fn test_iter_no_matching_groups() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A").unwrap();

    let mut it = caps.iter();
    let first = it.next();
    let second = it.next();
    let third = it.next();
    let fourth = it.next();
    let fifth = it.next();

    // Call the iterator to exercise the code
    drop(first);
    drop(second);
    drop(third);
    drop(fourth);
    drop(fifth);
}

#[test]
fn test_iter_multiple_matches() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A2C and B3D").unwrap();

    let mut it = caps.iter();
    let first = it.next();
    let second = it.next();
    let third = it.next();
    let fourth = it.next();
    let fifth = it.next();

    // Call the iterator to exercise the code
    drop(first);
    drop(second);
    drop(third);
    drop(fourth);
    drop(fifth);
}

