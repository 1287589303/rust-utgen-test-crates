// Answer 0

#[test]
fn test_from_non_empty_string() {
    let input: &str = "test string";
    let deserializer: StrDeserializer<Error> = input.from();
}

#[test]
fn test_from_empty_string() {
    let input: &str = "";
    let deserializer: StrDeserializer<Error> = input.from();
}

