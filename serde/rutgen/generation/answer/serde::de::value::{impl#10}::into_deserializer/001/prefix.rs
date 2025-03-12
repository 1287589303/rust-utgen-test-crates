// Answer 0

#[test]
fn test_into_deserializer_zero() {
    struct TestStruct {
        value: u32,
    }
    let input = TestStruct { value: 0 };
    let _deserializer: U32Deserializer<Error> = input.into_deserializer();
}

#[test]
fn test_into_deserializer_upper_boundary() {
    struct TestStruct {
        value: u32,
    }
    let input = TestStruct { value: 4294967295 };
    let _deserializer: U32Deserializer<Error> = input.into_deserializer();
}

#[test]
fn test_into_deserializer_mid_range() {
    struct TestStruct {
        value: u32,
    }
    let input = TestStruct { value: 2147483648 };
    let _deserializer: U32Deserializer<Error> = input.into_deserializer();
}

#[test]
fn test_into_deserializer_smallest_nonzero() {
    struct TestStruct {
        value: u32,
    }
    let input = TestStruct { value: 1 };
    let _deserializer: U32Deserializer<Error> = input.into_deserializer();
}

#[test]
fn test_into_deserializer_large_value() {
    struct TestStruct {
        value: u32,
    }
    let input = TestStruct { value: 3000000000 };
    let _deserializer: U32Deserializer<Error> = input.into_deserializer();
}

