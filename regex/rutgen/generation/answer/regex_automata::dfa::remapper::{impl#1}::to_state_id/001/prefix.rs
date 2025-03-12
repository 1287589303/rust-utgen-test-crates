// Answer 0

#[test]
fn test_to_state_id_valid_cases() {
    let mapper = IndexMapper { stride2: 0 };
    let state_id = mapper.to_state_id(0);
    
    let mapper_1 = IndexMapper { stride2: 1 };
    let state_id_1 = mapper_1.to_state_id(1);
    
    let mapper_63 = IndexMapper { stride2: 63 };
    let state_id_63 = mapper_63.to_state_id(1);
    
    let mapper_large = IndexMapper { stride2: 63 };
    let state_id_large = mapper_large.to_state_id((u64::MAX >> 1) as usize);
}

#[test]
#[should_panic]
fn test_to_state_id_invalid_large_index() {
    let mapper = IndexMapper { stride2: 0 };
    let _state_id = mapper.to_state_id(u64::MAX as usize);
}

#[test]
fn test_to_state_id_with_stride2() {
    let mapper_3 = IndexMapper { stride2: 3 };
    let state_id_3 = mapper_3.to_state_id(1);
    
    let mapper_5 = IndexMapper { stride2: 5 };
    let state_id_5 = mapper_5.to_state_id(2);
}

