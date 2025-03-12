// Answer 0

#[test]
fn test_set_lower_min_value() {
    let mut range = ClassBytesRange::default();
    range.set_lower(0);
}

#[test]
fn test_set_lower_mid_value() {
    let mut range = ClassBytesRange::default();
    range.set_lower(128);
}

#[test]
fn test_set_lower_max_value() {
    let mut range = ClassBytesRange::default();
    range.set_lower(255);
}

#[test]
fn test_set_lower_exceed_min_value() {
    let mut range = ClassBytesRange::default();
    range.set_lower(1);
}

#[test]
fn test_set_lower_exceed_max_value() {
    let mut range = ClassBytesRange::default();
    range.set_lower(254);
}

