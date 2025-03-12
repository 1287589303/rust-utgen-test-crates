// Answer 0

#[test]
fn test_serialize_str_non_empty() {
    let serializer = ContentSerializer::<()>::default();
    let input = "Hello, world!";
    let _ = serializer.serialize_str(input);
}

#[test]
fn test_serialize_str_empty() {
    let serializer = ContentSerializer::<()>::default();
    let input = "";
    let _ = serializer.serialize_str(input);
}

#[test]
fn test_serialize_str_special_characters() {
    let serializer = ContentSerializer::<()>::default();
    let input = "Special characters !@#$%^&*()";
    let _ = serializer.serialize_str(input);
}

#[test]
fn test_serialize_str_max_length_255() {
    let serializer = ContentSerializer::<()>::default();
    let input = "a".repeat(255);
    let _ = serializer.serialize_str(&input);
}

#[test]
fn test_serialize_str_max_length_4096() {
    let serializer = ContentSerializer::<()>::default();
    let input = "a".repeat(4096);
    let _ = serializer.serialize_str(&input);
}

