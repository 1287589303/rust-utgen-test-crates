// Answer 0

#[test]
fn test_serialize_seq_none() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(None);
}

#[test]
fn test_serialize_seq_zero() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(0));
}

#[test]
fn test_serialize_seq_one() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(1));
}

#[test]
fn test_serialize_seq_one_hundred() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(100));
}

#[test]
fn test_serialize_seq_max_usize() {
    let serializer = Serializer;
    let result = serializer.serialize_seq(Some(std::usize::MAX));
}

