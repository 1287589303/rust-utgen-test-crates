// Answer 0

#[test]
fn test_serialize_unit() {
    let serializer = Serializer;
    let result = serializer.serialize_unit();
}

#[test]
fn test_serialize_unit_multiple_calls() {
    let serializer = Serializer;
    let result1 = serializer.serialize_unit();
    let result2 = serializer.serialize_unit();
}

