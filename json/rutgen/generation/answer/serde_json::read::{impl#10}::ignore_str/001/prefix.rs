// Answer 0

#[test]
fn test_ignore_str_valid() {
    let slice: &[u8] = b"valid input";
    let mut delegate = SliceRead {
        slice,
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };

    let mut read = StrRead {
        delegate,
        #[cfg(feature = "raw_value")]
        data: "valid input",
    };

    let result = read.ignore_str();
}

#[test]
fn test_ignore_str_non_empty_slice() {
    let slice: &[u8] = b"another valid input";
    let mut delegate = SliceRead {
        slice,
        index: 0,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };

    let mut read = StrRead {
        delegate,
        #[cfg(feature = "raw_value")]
        data: "another valid input",
    };

    let result = read.ignore_str();
}

#[test]
fn test_ignore_str_index_within_bounds() {
    let slice: &[u8] = b"more valid input";
    let mut delegate = SliceRead {
        slice,
        index: 5,
        #[cfg(feature = "raw_value")]
        raw_buffering_start_index: 0,
    };

    let mut read = StrRead {
        delegate,
        #[cfg(feature = "raw_value")]
        data: "more valid input",
    };

    let result = read.ignore_str();
}

