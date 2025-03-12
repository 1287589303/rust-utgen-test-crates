// Answer 0

#[test]
fn test_serialize_seq_none() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_seq(None);
}

#[test]
fn test_serialize_seq_some_zero() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_seq(Some(0));
}

#[test]
fn test_serialize_seq_some_one() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_seq(Some(1));
}

#[test]
fn test_serialize_seq_some_ten() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_seq(Some(10));
}

#[test]
fn test_serialize_seq_some_max() {
    let serializer = ContentSerializer::<()>::default();
    let result = serializer.serialize_seq(Some(usize::MAX));
}

