// Answer 0

#[test]
fn test_end_zero_to_zero() {
    let range = ClassBytesRange::new(0, 0);
    let result = range.end();
}

#[test]
fn test_end_one_to_two_fifty_five() {
    let range = ClassBytesRange::new(1, 255);
    let result = range.end();
}

#[test]
fn test_end_two_fifty_five_to_two_fifty_five() {
    let range = ClassBytesRange::new(255, 255);
    let result = range.end();
}

#[test]
#[should_panic]
fn test_end_two_fifty_five_to_zero() {
    let range = ClassBytesRange::new(255, 0);
    let result = range.end();
}

#[test]
fn test_end_one_hundred_to_two_hundred() {
    let range = ClassBytesRange::new(100, 200);
    let result = range.end();
}

