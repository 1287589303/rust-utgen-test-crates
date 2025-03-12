// Answer 0

#[test]
fn test_from_bytes_unchecked_valid() {
    let state_len: u32 = 2; // should be > 0
    let stride2: u32 = 9; // bound is 9
    let alphabet_len: u32 = 512; // 2^9 = 512, thus classes.alphabet_len() = stride
    let table_len: usize = (state_len as usize) * (1 << stride2) * 4; // StateID::SIZE is 4
    let mut slice: Vec<u8> = vec![0; 300 + table_len]; // ensuring the slice length is >= 300
    let mut cursor = 0;

    // Write state_len
    cursor += wire::try_read_u32_as_usize(&mut slice[cursor..], "state length").unwrap().1;
    slice[..4].copy_from_slice(&state_len.to_le_bytes());

    // Write stride2
    cursor += wire::try_read_u32_as_usize(&mut slice[cursor..], "stride2").unwrap().1;
    slice[4..8].copy_from_slice(&stride2.to_le_bytes());

    // Write ByteClasses
    let byte_classes = [0u8; 256]; // Placeholder for byte class since we need 256 bytes
    cursor += ByteClasses::from_bytes(&slice[cursor..]).unwrap().1;
    slice[8..264].copy_from_slice(&byte_classes);

    // Calculate the actual number of transitions and write them
    let num_transitions = (state_len as usize) * (1 << stride2);
    for i in 0..num_transitions {
        slice[264 + (i * 4)..264 + ((i + 1) * 4)].copy_from_slice(&(i as u32).to_le_bytes());
    }

    let result = unsafe { TransitionTable::from_bytes_unchecked(&mut slice) };
}

#[test]
fn test_from_bytes_unchecked_stride2_equals_min() {
    let state_len: u32 = 2; // should be > 0
    let stride2: u32 = 1; // bound is 1
    let alphabet_len: u32 = 2; // 1 << 1 = 2, thus classes.alphabet_len() = stride
    let table_len: usize = (state_len as usize) * (1 << stride2) * 4; // StateID::SIZE is 4
    let mut slice: Vec<u8> = vec![0; 300 + table_len]; // ensuring the slice length is >= 300
    let mut cursor = 0;

    // Write state_len
    cursor += wire::try_read_u32_as_usize(&mut slice[cursor..], "state length").unwrap().1;
    slice[..4].copy_from_slice(&state_len.to_le_bytes());

    // Write stride2
    cursor += wire::try_read_u32_as_usize(&mut slice[cursor..], "stride2").unwrap().1;
    slice[4..8].copy_from_slice(&stride2.to_le_bytes());

    // Write ByteClasses
    let byte_classes = [0u8; 256];
    cursor += ByteClasses::from_bytes(&slice[cursor..]).unwrap().1;
    slice[8..264].copy_from_slice(&byte_classes);

    // Calculate the actual number of transitions and write them
    let num_transitions = (state_len as usize) * (1 << stride2);
    for i in 0..num_transitions {
        slice[264 + (i * 4)..264 + ((i + 1) * 4)].copy_from_slice(&(i as u32).to_le_bytes());
    }

    let result = unsafe { TransitionTable::from_bytes_unchecked(&mut slice) };
}

