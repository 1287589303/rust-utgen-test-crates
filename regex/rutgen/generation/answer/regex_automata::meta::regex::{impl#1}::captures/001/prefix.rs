// Answer 0

#[test]
fn test_captures_valid_input() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let mut caps = Captures {
        group_info: GroupInfo::new(),
        pid: Some(PatternID::new(0)),
        slots: vec![None; 6],
    };
    re.captures("2010-03-14", &mut caps);
}

#[test]
fn test_captures_valid_minimum_date() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let mut caps = Captures {
        group_info: GroupInfo::new(),
        pid: Some(PatternID::new(1)),
        slots: vec![None; 6],
    };
    re.captures("0000-00-00", &mut caps);
}

#[test]
fn test_captures_invalid_input_format() {
    let re = Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let mut caps = Captures {
        group_info: GroupInfo::new(),
        pid: None,
        slots: vec![None; 6],
    };
    re.captures("2010/03/14", &mut caps);
}

