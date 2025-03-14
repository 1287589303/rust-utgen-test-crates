// Answer 0

#[test]
fn test_new_with_padding_index() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }

    impl TestStruct {
        pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
            Self {
                decoded_len: decoded_bytes,
                padding_offset: padding_index,
            }
        }
    }

    let instance = TestStruct::new(10, Some(2));
    assert_eq!(instance.decoded_len, 10);
    assert_eq!(instance.padding_offset, Some(2));
}

#[test]
fn test_new_without_padding_index() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }

    impl TestStruct {
        pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
            Self {
                decoded_len: decoded_bytes,
                padding_offset: padding_index,
            }
        }
    }

    let instance = TestStruct::new(5, None);
    assert_eq!(instance.decoded_len, 5);
    assert_eq!(instance.padding_offset, None);
}

#[test]
fn test_new_with_zero_bytes() {
    struct TestStruct {
        decoded_len: usize,
        padding_offset: Option<usize>,
    }

    impl TestStruct {
        pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
            Self {
                decoded_len: decoded_bytes,
                padding_offset: padding_index,
            }
        }
    }

    let instance = TestStruct::new(0, Some(0));
    assert_eq!(instance.decoded_len, 0);
    assert_eq!(instance.padding_offset, Some(0));
}

