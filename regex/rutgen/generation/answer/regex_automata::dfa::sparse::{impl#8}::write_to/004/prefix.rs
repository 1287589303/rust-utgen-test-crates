// Answer 0

#[test]
fn test_write_to_valid_case() {
    #[derive(Clone)]
    struct TestEndian;
    impl Endian for TestEndian {
        fn write_u32(_: u32, dst: &mut [u8]) {
            dst[0..4].copy_from_slice(&u32::to_le_bytes(_) );
        }
    }

    let sparse_data: &[u8] = &[0u8; 64]; // example byte slice
    let classes = ByteClasses::singletons();
    let transitions = Transitions {
        sparse: sparse_data,
        classes,
        state_len: 4,
        pattern_len: 2,
    };

    let mut buffer = vec![0u8; transitions.write_to_len()];
    let result = transitions.write_to::<TestEndian>(&mut buffer).unwrap();
    
    assert_eq!(result, buffer.len());
}

#[test]
fn test_write_to_last_state() {
    #[derive(Clone)]
    struct TestEndian;
    impl Endian for TestEndian {
        fn write_u32(_: u32, dst: &mut [u8]) {
            dst[0..4].copy_from_slice(&u32::to_le_bytes(_) );
        }
    }

    let sparse_data: &[u8] = &[0u8; 64]; // example byte slice
    let classes = ByteClasses::singletons();
    let transitions = Transitions {
        sparse: sparse_data,
        classes,
        state_len: 4,
        pattern_len: 2,
    };

    let mut buffer = vec![0u8; transitions.write_to_len()];
    let result = transitions.write_to::<TestEndian>(&mut buffer).unwrap();

    assert_eq!(result, buffer.len());
}

