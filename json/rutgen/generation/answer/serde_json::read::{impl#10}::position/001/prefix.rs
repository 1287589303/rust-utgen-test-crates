// Answer 0

#[test]
fn test_position_valid_slice_read() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 0;
    let delegate = SliceRead { slice, index };

    let str_read = StrRead { delegate };

    let _ = str_read.position();
}

#[test]
fn test_position_slice_read_at_middle() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 2;
    let delegate = SliceRead { slice, index };

    let str_read = StrRead { delegate };

    let _ = str_read.position();
}

#[test]
fn test_position_slice_read_at_end() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 5;
    let delegate = SliceRead { slice, index };

    let str_read = StrRead { delegate };

    let _ = str_read.position();
}

#[test]
#[should_panic]
fn test_position_slice_read_out_of_bounds() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 6;  // Index is out of bounds

    let delegate = SliceRead { slice, index };

    let str_read = StrRead { delegate };

    let _ = str_read.position();
}

