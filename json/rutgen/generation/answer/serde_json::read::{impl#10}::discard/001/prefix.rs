// Answer 0

#[test]
fn test_discard_with_non_empty_slice() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 0;
    let delegate = SliceRead { slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

#[test]
fn test_discard_with_middle_index() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 2;
    let delegate = SliceRead { slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

#[test]
fn test_discard_with_last_index() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 5;
    let delegate = SliceRead { slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

#[test]
fn test_discard_with_large_slice() {
    let slice: Vec<u8> = (0..4096).map(|x| x as u8).collect();
    let index = 0;
    let delegate = SliceRead { slice: &slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

#[test]
#[cfg(feature = "raw_value")]
fn test_discard_with_raw_buffering() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 0;
    let raw_buffering_start_index = 0;
    let delegate = SliceRead { slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

#[test]
#[cfg(feature = "raw_value")]
fn test_discard_with_raw_buffering_middle_index() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let index = 2;
    let raw_buffering_start_index = 2;
    let delegate = SliceRead { slice, index };
    let mut str_read = StrRead { delegate };

    str_read.discard();
}

