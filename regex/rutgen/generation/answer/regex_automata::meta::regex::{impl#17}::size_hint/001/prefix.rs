// Answer 0

#[test]
fn test_size_hint_zero_limit() {
    let limit: usize = 0;
    let splits = Split {
        partitions: [SplitRange { start: 0, end: 0 }; 3],
        len: 0,
    };
    let split_n = SplitN { splits, limit };
    split_n.size_hint();
}

#[test]
fn test_size_hint_max_limit() {
    let limit: usize = usize::MAX;
    let splits = Split {
        partitions: [SplitRange { start: 0, end: 0 }; 3],
        len: 0,
    };
    let split_n = SplitN { splits, limit };
    split_n.size_hint();
}

#[test]
fn test_size_hint_large_limit() {
    let limit: usize = 1_000_000;
    let splits = Split {
        partitions: [SplitRange { start: 0, end: 0 }; 3],
        len: 0,
    };
    let split_n = SplitN { splits, limit };
    split_n.size_hint();
}

