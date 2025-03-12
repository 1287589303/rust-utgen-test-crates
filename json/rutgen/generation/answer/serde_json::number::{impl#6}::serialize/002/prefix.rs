// Answer 0

#[test]
fn test_serialize_neg_int_min() {
    let number = Number { n: N::NegInt(-9223372036854775808) };
    let serializer = ...; // create a suitable serializer
    let _ = number.serialize(serializer);
}

#[test]
fn test_serialize_neg_int_mid() {
    let number = Number { n: N::NegInt(-1234567890) };
    let serializer = ...; // create a suitable serializer
    let _ = number.serialize(serializer);
}

#[test]
fn test_serialize_neg_int_max() {
    let number = Number { n: N::NegInt(-1) };
    let serializer = ...; // create a suitable serializer
    let _ = number.serialize(serializer);
}

