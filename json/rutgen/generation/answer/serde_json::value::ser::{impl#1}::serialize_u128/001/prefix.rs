// Answer 0

#[test]
fn test_serialize_u128_success() {
    let serializer = Serializer;
    let value: u128 = 12345678901234567890; // within 0 to 2^64-1
    let _result = serializer.serialize_u128(value);
}

#[test]
#[should_panic]
fn test_serialize_u128_out_of_range() {
    let serializer = Serializer;
    let value: u128 = 18446744073709551616; // equal to 2^64, should trigger the error
    let _result = serializer.serialize_u128(value);
}

#[test]
#[should_panic]
fn test_serialize_u128_large_value() {
    let serializer = Serializer;
    let value: u128 = u128::MAX; // greater than 2^64-1, should trigger the error
    let _result = serializer.serialize_u128(value);
}

