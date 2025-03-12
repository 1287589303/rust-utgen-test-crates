// Answer 0

#[test]
fn test_extract_with_three_capture_groups() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "On 2010-03-14, I became a Tenneessee lamb.";
    let captures = re.captures(hay).unwrap();
    let (full, [year, month, day]) = captures.extract::<3>();
}

#[test]
fn test_extract_with_empty_haystack() {
    let re = Regex::new(r"(a)(b)(c)").unwrap();
    let hay = "";
    let captures = re.captures(hay);
    assert!(captures.is_none());
}

#[test]
fn test_extract_with_no_matches() {
    let re = Regex::new(r"(a)(b)(c)").unwrap();
    let hay = "xyz";
    let captures = re.captures(hay);
    assert!(captures.is_none());
}

#[test]
fn test_extract_with_multiple_matches() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "1973-01-05, 1975-08-25 and 1980-10-18";
    let mut dates: Vec<(&str, &str, &str)> = vec![];
    for captures in re.captures_iter(hay) {
        let (full, [year, month, day]) = captures.extract::<3>();
        dates.push((year, month, day));
    }
}

#[test]
fn test_extract_with_single_capture_group() {
    let re = Regex::new(r"(\d+)").unwrap();
    let hay = "There are 42 apples.";
    let captures = re.captures(hay).unwrap();
    let (full, [number]) = captures.extract::<1>();
}

#[test]
#[should_panic]
fn test_extract_with_incorrect_n_capture_groups() {
    let re = Regex::new(r"([a-z]+)-([0-9]+)").unwrap();
    let hay = "foo-42";
    let captures = re.captures(hay).unwrap();
    let (_, [string]) = captures.extract::<1>(); // This should panic since N != number of groups
}

