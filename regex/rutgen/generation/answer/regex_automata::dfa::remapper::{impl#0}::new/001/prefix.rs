// Answer 0

#[test]
fn test_remapper_new_valid_input() {
    struct ValidRemappable {
        length: usize,
        stride: usize,
    }
    
    impl ValidRemappable {
        fn new(length: usize, stride: usize) -> Self {
            ValidRemappable { length, stride }
        }
    }
    
    impl Remappable for ValidRemappable {
        fn state_len(&self) -> usize {
            self.length
        }
        
        fn stride2(&self) -> usize {
            self.stride
        }
        
        fn to_state_id(&self, index: usize) -> StateID {
            StateID(index as SmallIndex)
        }
    }
    
    let remappable = ValidRemappable::new(10, 2);
    let remapper = Remapper::new(&remappable);
}

#[test]
fn test_remapper_new_boundary_input_zero_length() {
    struct ZeroLengthRemappable;
    
    impl Remappable for ZeroLengthRemappable {
        fn state_len(&self) -> usize {
            0
        }
        
        fn stride2(&self) -> usize {
            1
        }
        
        fn to_state_id(&self, _: usize) -> StateID {
            StateID(0)
        }
    }

    let remappable = ZeroLengthRemappable;
    let remapper = Remapper::new(&remappable);
}

#[test]
#[should_panic]
fn test_remapper_new_boundary_input_negative_stride() {
    struct NegativeStrideRemappable;
    
    impl Remappable for NegativeStrideRemappable {
        fn state_len(&self) -> usize {
            5
        }
        
        fn stride2(&self) -> usize {
            usize::MAX // let's consider the max as exceeding a valid range
        }
        
        fn to_state_id(&self, index: usize) -> StateID {
            StateID(index as SmallIndex)
        }
    }

    let remappable = NegativeStrideRemappable;
    let remapper = Remapper::new(&remappable);
}

#[test]
fn test_remapper_new_valid_input_zero_stride() {
    struct ZeroStrideRemappable {
        length: usize,
    }
    
    impl Remappable for ZeroStrideRemappable {
        fn state_len(&self) -> usize {
            self.length
        }
        
        fn stride2(&self) -> usize {
            0
        }
        
        fn to_state_id(&self, index: usize) -> StateID {
            StateID(index as SmallIndex)
        }
    }

    let remappable = ZeroStrideRemappable { length: 5 };
    let remapper = Remapper::new(&remappable);
}

