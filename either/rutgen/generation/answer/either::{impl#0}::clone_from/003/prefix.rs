// Answer 0

#[test]
fn test_clone_from_both_right_variants() {
    struct TestData(String);

    let mut dest = Either::Right(TestData("Initial".to_string()));
    let source = Either::Right(TestData("Cloned".to_string()));

    dest.clone_from(&source);
}

#[test]
fn test_clone_from_both_right_variants_with_different_data() {
    struct TestData(String);

    let mut dest = Either::Right(TestData("Old Value".to_string()));
    let source = Either::Right(TestData("New Value".to_string()));

    dest.clone_from(&source);
}

#[test]
fn test_clone_from_right_variant_with_empty_string() {
    struct TestData(String);

    let mut dest = Either::Right(TestData("Not Empty".to_string()));
    let source = Either::Right(TestData("".to_string()));

    dest.clone_from(&source);
}

#[test]
fn test_clone_from_right_variant_with_long_string() {
    struct TestData(String);

    let mut dest = Either::Right(TestData("Short".to_string()));
    let source = Either::Right(TestData("A very long string that exceeds usual lengths.".to_string()));

    dest.clone_from(&source);
}

