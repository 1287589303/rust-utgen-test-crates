// Answer 0

#[test]
fn test_captures_valid_date() {
    let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, "2022-08-15", &mut caps);
}

#[test]
fn test_captures_leap_year() {
    let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, "2020-02-29", &mut caps);
}

#[test]
fn test_captures_boundary_date() {
    let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, "0000-01-01", &mut caps);
    re.captures(&mut cache, "9999-12-31", &mut caps);
}

#[test]
fn test_captures_invalid_date() {
    let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, "2022-13-01", &mut caps);
    re.captures(&mut cache, "2022-00-15", &mut caps);
    re.captures(&mut cache, "2022-02-30", &mut caps);
    re.captures(&mut cache, "2022-07-32", &mut caps);
}

