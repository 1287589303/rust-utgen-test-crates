// Answer 0

#[test]
fn test_serialize_i128_success_u64() {
    let serializer = Serializer;
    let value: i128 = 0; // minimum value that passes u64::try_from
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_success_i64() {
    let serializer = Serializer;
    let value: i128 = i64::MAX as i128; // maximum value that passes i64::try_from
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_out_of_range_positive() {
    let serializer = Serializer;
    let value: i128 = u128::MAX as i128; // value above the i64 range to trigger error
    let _result = serializer.serialize_i128(value);
}

#[test]
fn test_serialize_i128_out_of_range_negative() {
    let serializer = Serializer;
    let value: i128 = i128::MIN; // value below the u64 range to trigger error
    let _result = serializer.serialize_i128(value);
}

