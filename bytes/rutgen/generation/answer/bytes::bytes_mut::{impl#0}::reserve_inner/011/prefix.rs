// Answer 0

#[test]
fn test_reserve_inner_kind_arc() {
    let mut bytes_mut = {
        let v = Vec::with_capacity(10);
        BytesMut::from_vec(v)
    };
    
    // Setting up preconditions.
    let additional = 5;
    unsafe { bytes_mut.reserve_inner(additional, true); }
}

#[test]
fn test_reserve_inner_capacity_not_enough() {
    let mut bytes_mut = {
        let v = Vec::with_capacity(12);
        BytesMut::from_vec(v)
    };

    // Setting up preconditions.
    let additional = 10;
    let _ = unsafe { bytes_mut.reserve_inner(additional, true) };
}

#[test]
fn test_reserve_inner_allocate_true() {
    let mut bytes_mut = {
        let v = Vec::with_capacity(20);
        BytesMut::from_vec(v)
    };

    // Setting up preconditions.
    let additional = 7;
    let _ = unsafe { bytes_mut.reserve_inner(additional, true) };
}

#[test]
fn test_reserve_inner_offset() {
    let mut bytes_mut = {
        let v = Vec::with_capacity(15);
        BytesMut::from_vec(v)
    };
    
    // Setting up preconditions.
    let additional = 4;
    unsafe { bytes_mut.reserve_inner(additional, true); }
}

