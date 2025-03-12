// Answer 0

#[test]
fn test_next_valid_case() {
    let mut slice_read = SliceRead {
        slice: &[1, 2, 3, 4, 5],
        index: 0,
    };
    let result = slice_read.next();
}

#[test]
fn test_next_middle_case() {
    let mut slice_read = SliceRead {
        slice: &[10, 20, 30, 40, 50],
        index: 2,
    };
    let result = slice_read.next();
}

#[test]
fn test_next_boundary_case() {
    let mut slice_read = SliceRead {
        slice: &[100, 200, 300],
        index: 1,
    };
    let result = slice_read.next();
}

