// Answer 0

#[test]
fn test_set_upper_lower_bound() {
    let mut range = ClassBytesRange::default();
    range.set_upper(0);
}

#[test]
fn test_set_upper_upper_bound() {
    let mut range = ClassBytesRange::default();
    range.set_upper(255);
}

#[test]
fn test_set_upper_change_from_lower_to_higher() {
    let mut range = ClassBytesRange { start: 10, end: 20 };
    range.set_upper(25);
}

#[test]
fn test_set_upper_change_from_higher_to_lower() {
    let mut range = ClassBytesRange { start: 10, end: 20 };
    range.set_upper(15);
}

#[test]
fn test_set_upper_no_change() {
    let mut range = ClassBytesRange { start: 10, end: 20 };
    range.set_upper(20);
}

