// Answer 0

#[test]
fn test_as_slice_len_zero() {
    let split = Split {
        partitions: [SplitRange::Old(Utf8Range::new(0, 0)), 
                     SplitRange::Old(Utf8Range::new(0, 0)), 
                     SplitRange::Old(Utf8Range::new(0, 0))],
        len: 0,
    };
    let _result = split.as_slice();
}

#[test]
fn test_as_slice_len_one() {
    let split = Split {
        partitions: [SplitRange::Old(Utf8Range::new(0, 1)), 
                     SplitRange::Old(Utf8Range::new(0, 0)), 
                     SplitRange::Old(Utf8Range::new(0, 0))],
        len: 1,
    };
    let _result = split.as_slice();
}

#[test]
fn test_as_slice_len_two() {
    let split = Split {
        partitions: [SplitRange::Old(Utf8Range::new(1, 2)), 
                     SplitRange::New(Utf8Range::new(3, 4)), 
                     SplitRange::Old(Utf8Range::new(0, 0))],
        len: 2,
    };
    let _result = split.as_slice();
}

#[test]
fn test_as_slice_len_three() {
    let split = Split {
        partitions: [SplitRange::Both(Utf8Range::new(0, 1)), 
                     SplitRange::New(Utf8Range::new(2, 3)), 
                     SplitRange::Old(Utf8Range::new(4, 5))],
        len: 3,
    };
    let _result = split.as_slice();
}

