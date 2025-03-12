// Answer 0

#[test]
fn test_from_bytes_unchecked_valid_input() {
    let state_len: u32 = 2;
    let idlen: u32 = 3;
    let pattern_id_size = std::mem::size_of::<u32>();
    
    let slices = [
        (0u32, 2u32),
        (2u32, 2u32),
    ];

    let pattern_ids = [1u32, 2u32, 3u32];

    let slice_len = 4 + (2 * slices.len() * pattern_id_size) + 4 + 4 + (pattern_ids.len() * pattern_id_size);
    let mut slice: Vec<u8> = vec![0; slice_len];
    let mut offset = 0;

    offset += slice[offset..].write_u32::<ByteOrder>(state_len).unwrap();
    for &(start, length) in &slices {
        offset += slice[offset..].write_u32::<ByteOrder>(start).unwrap();
        offset += slice[offset..].write_u32::<ByteOrder>(length).unwrap();
    }

    offset += slice[offset..].write_u32::<ByteOrder>(idlen).unwrap();
    offset += slice[offset..].write_u32::<ByteOrder>(pattern_ids.len() as u32).unwrap();
    for &id in &pattern_ids {
        offset += slice[offset..].write_u32::<ByteOrder>(id).unwrap();
    }

    let result = unsafe { from_bytes_unchecked(&slice) };
}

#[test]
fn test_from_bytes_unchecked_minimum_valid_input() {
    let state_len: u32 = 1;
    let idlen: u32 = 1;
    let pattern_id_size = std::mem::size_of::<u32>();
    
    let slices = [
        (0u32, 1u32),
    ];

    let pattern_ids = [0u32];

    let slice_len = 4 + (2 * slices.len() * pattern_id_size) + 4 + 4 + (pattern_ids.len() * pattern_id_size);
    let mut slice: Vec<u8> = vec![0; slice_len];
    let mut offset = 0;

    offset += slice[offset..].write_u32::<ByteOrder>(state_len).unwrap();
    for &(start, length) in &slices {
        offset += slice[offset..].write_u32::<ByteOrder>(start).unwrap();
        offset += slice[offset..].write_u32::<ByteOrder>(length).unwrap();
    }
    
    offset += slice[offset..].write_u32::<ByteOrder>(idlen).unwrap();
    offset += slice[offset..].write_u32::<ByteOrder>(pattern_ids.len() as u32).unwrap();
    for &id in &pattern_ids {
        offset += slice[offset..].write_u32::<ByteOrder>(id).unwrap();
    }

    let result = unsafe { from_bytes_unchecked(&slice) };
}

#[test]
fn test_from_bytes_unchecked_exact_boundary() {
    let state_len: u32 = 2;
    let idlen: u32 = 1;
    let pattern_id_size = std::mem::size_of::<u32>();

    let slices = [
        (0u32, 1u32),
        (1u32, 1u32),
    ];

    let pattern_ids = [0u32];

    let slice_len = 4 + (2 * slices.len() * pattern_id_size) + 4 + 4 + (pattern_ids.len() * pattern_id_size);
    let mut slice: Vec<u8> = vec![0; slice_len];
    let mut offset = 0;

    offset += slice[offset..].write_u32::<ByteOrder>(state_len).unwrap();
    for &(start, length) in &slices {
        offset += slice[offset..].write_u32::<ByteOrder>(start).unwrap();
        offset += slice[offset..].write_u32::<ByteOrder>(length).unwrap();
    }

    offset += slice[offset..].write_u32::<ByteOrder>(idlen).unwrap();
    offset += slice[offset..].write_u32::<ByteOrder>(pattern_ids.len() as u32).unwrap();
    for &id in &pattern_ids {
        offset += slice[offset..].write_u32::<ByteOrder>(id).unwrap();
    }

    let result = unsafe { from_bytes_unchecked(&slice) };
}

#[test]
#[should_panic]
fn test_from_bytes_unchecked_insufficient_length() {
    let slice: &[u8] = &[];
    let result = unsafe { from_bytes_unchecked(slice) };
}

