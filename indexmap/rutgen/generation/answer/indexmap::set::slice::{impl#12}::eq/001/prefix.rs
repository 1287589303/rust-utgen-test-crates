// Answer 0

#[test]
fn test_eq_equal() {
    struct TestBucket {
        hash: u64, // simplified for the example
        key: i32,
        value: i32,
    }

    let slice = Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 2, value: 20 },
        ],
    };
    let other: &[i32; 2] = &[1, 2];

    let _ = slice.eq(other);
}

#[test]
fn test_eq_not_equal() {
    struct TestBucket {
        hash: u64, // simplified for the example
        key: i32,
        value: i32,
    }

    let slice = Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 3, value: 30 },
        ],
    };
    let other: &[i32; 2] = &[1, 2];

    let _ = slice.eq(other);
}

#[test]
fn test_eq_empty_slice_other_non_empty() {
    struct TestBucket {
        hash: u64, // simplified for the example
        key: i32,
        value: i32,
    }

    let slice: Slice<i32> = Slice { entries: [] };
    let other: &[i32; 1] = &[1];

    let _ = slice.eq(other);
}

#[test]
fn test_eq_non_empty_slice_other_empty() {
    struct TestBucket {
        hash: u64, // simplified for the example
        key: i32,
        value: i32,
    }

    let slice = Slice {
        entries: [
            Bucket { hash: 1, key: 1, value: 10 },
            Bucket { hash: 2, key: 2, value: 20 },
        ],
    };
    let other: &[i32; 0] = &[];

    let _ = slice.eq(other);
}

#[test]
fn test_eq_boundary_cases() {
    struct TestBucket {
        hash: u64, // simplified for the example
        key: i32,
        value: i32,
    }

    let slice = Slice {
        entries: [
            Bucket { hash: 1, key: 0, value: 10 },
            Bucket { hash: 2, key: 0, value: 20 },
        ],
    };
    let other: &[i32; 2] = &[0, 0];

    let _ = slice.eq(other);
}

