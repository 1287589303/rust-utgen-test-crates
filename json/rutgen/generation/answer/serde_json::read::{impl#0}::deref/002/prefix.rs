// Answer 0

#[test]
fn test_deref_borrowed() {
    let value: i32 = 10;
    let reference = Reference::Borrowed(&value);
    let _result = reference.deref(); // Test for Reference::Borrowed
}

#[test]
fn test_deref_copied() {
    let value: i32 = 10;
    let reference = Reference::Copied(&value);
    let _result = reference.deref(); // Test for Reference::Copied
}

