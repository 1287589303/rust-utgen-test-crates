// Answer 0

#[test]
fn test_borrowed_str_deserializer_from_non_empty_string() {
    let test_str: &str = "test string";
    let borrowed = Borrowed(test_str);
    let deserializer: BorrowedStrDeserializer<Error> = borrowed.from();
}

#[test]
fn test_borrowed_str_deserializer_from_single_character() {
    let test_str: &str = "a";
    let borrowed = Borrowed(test_str);
    let deserializer: BorrowedStrDeserializer<Error> = borrowed.from();
}

#[test]
fn test_borrowed_str_deserializer_from_empty_string() {
    let test_str: &str = "";
    let borrowed = Borrowed(test_str);
    let deserializer: BorrowedStrDeserializer<Error> = borrowed.from();
}

#[test]
fn test_borrowed_str_deserializer_from_whitespace_string() {
    let test_str: &str = "   ";
    let borrowed = Borrowed(test_str);
    let deserializer: BorrowedStrDeserializer<Error> = borrowed.from();
}

