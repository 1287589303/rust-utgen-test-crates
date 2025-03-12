// Answer 0

#[test]
fn test_extract_valid_case() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "On 2010-03-14, I became a Tenneessee lamb.";
    let captures = re.captures(hay).unwrap();
    let (full, [year, month, day]) = captures.extract::<3>();
}

#[test]
fn test_extract_empty_string() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "";
    let captures = re.captures(hay);
    if let Some(captures) = captures {
        let result = captures.extract::<3>();
    }
}

#[test]
fn test_extract_no_matches() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "No date here.";
    let captures = re.captures(hay);
    if let Some(captures) = captures {
        let result = captures.extract::<3>();
    }
}

#[test]
#[should_panic]
fn test_extract_incorrect_number_of_groups() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})").unwrap();
    let hay = "2010-03";
    let captures = re.captures(hay).unwrap();
    let _ = captures.extract::<3>();
}

