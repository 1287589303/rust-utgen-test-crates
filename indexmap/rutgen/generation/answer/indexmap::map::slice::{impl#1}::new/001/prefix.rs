// Answer 0

#[test]
fn test_new_empty_slice_u32_string() {
    struct TestBucket {
        key: u32,
        value: String,
    }

    let empty_slice: &Slice<u32, String> = Slice::new();
}

#[test]
fn test_new_empty_slice_string_f64() {
    struct TestBucket {
        key: String,
        value: f64,
    }

    let empty_slice: &Slice<String, f64> = Slice::new();
}

#[test]
fn test_new_empty_slice_char_bool() {
    struct TestBucket {
        key: char,
        value: bool,
    }

    let empty_slice: &Slice<char, bool> = Slice::new();
}

