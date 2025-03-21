// Answer 0

#[test]
fn test_write_to_exact_buffer_size() {
    use crate::util::wire::{self, Endian};
    
    struct TestEndian;
    
    impl Endian for TestEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let kind = StartKind::Both;

    let start_map = StartByteMap {
        map: [Start::default(); 256],
    };

    let stride = 4;
    let pattern_len = Some(2);
    let universal_start_unanchored = Some(StateID(1));
    let universal_start_anchored = Some(StateID(2));

    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3)];
    
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let nwrite = start_table.write_to_len();
    let mut dst = vec![0u8; nwrite];

    // This call to start_map.write_to returns Ok
    start_table.write_to::<TestEndian>(&mut dst).unwrap();
}

#[test]
fn test_write_to_invalid_start_byte_map() {
    use crate::util::wire::{self, Endian};
    
    struct TestEndian;
    
    impl Endian for TestEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let kind = StartKind::Unanchored;

    // Using a valid StartKind but invalid StartByteMap
    let start_map = StartByteMap {
        map: [Start::default(); 256],
    };

    let stride = 4;
    let pattern_len = None; // Test case where patterns are not provided
    let universal_start_unanchored = None;
    let universal_start_anchored = None;

    let table = vec![StateID(0), StateID(1)];
    
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let nwrite = start_table.write_to_len();
    let mut dst = vec![0u8; nwrite];

    // Expecting this call to start_map.write_to to return Err
    assert!(start_table.write_to::<TestEndian>(&mut dst).is_err());
}

