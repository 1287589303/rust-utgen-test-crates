// Answer 0

#[test]
fn test_write_to_boundary_case_equal_length() {
    let transitions = Transitions {
        sparse: vec![0u8; 20],
        classes: ByteClasses::empty(),
        state_len: 5,
        pattern_len: 3,
    };

    let nwrite = transitions.write_to_len();
    let mut dst = vec![0u8; nwrite];

    let result = transitions.write_to::<Endian>(&mut dst);
}

#[test]
fn test_write_to_classes_write_error() {
    struct FailByteClasses;
    impl ByteClasses {
        pub fn write_to(&self, _: &mut [u8]) -> Result<usize, SerializeError> {
            Err(SerializeError::buffer_too_small("byte class map"))
        }
    }

    let transitions = Transitions {
        sparse: vec![0u8; 20],
        classes: FailByteClasses,
        state_len: 5,
        pattern_len: 3,
    };

    let nwrite = transitions.write_to_len();
    let mut dst = vec![0u8; nwrite];

    let result = transitions.write_to::<Endian>(&mut dst);
}

