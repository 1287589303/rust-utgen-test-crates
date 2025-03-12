// Answer 0

#[test]
fn test_peek_position_valid_index() {
    let data: &[u8] = b"Sample data for testing";
    let delegate = SliceRead { slice: data, index: 5 };
    let read_instance = StrRead { delegate };

    let position = read_instance.peek_position();
}

#[test]
fn test_peek_position_zero_index() {
    let data: &[u8] = b"Sample data for testing";
    let delegate = SliceRead { slice: data, index: 0 };
    let read_instance = StrRead { delegate };

    let position = read_instance.peek_position();
}

#[test]
fn test_peek_position_end_of_slice() {
    let data: &[u8] = b"Sample data for testing";
    let delegate = SliceRead { slice: data, index: data.len() - 1 };
    let read_instance = StrRead { delegate };

    let position = read_instance.peek_position();
}

#[test]
fn test_peek_position_out_of_bounds_index() {
    let data: &[u8] = b"Sample data for testing";
    let delegate = SliceRead { slice: data, index: data.len() };
    let read_instance = StrRead { delegate };

    let position = read_instance.peek_position();
}

