// Answer 0

#[test]
fn test_ctrl_valid_index_start() {
    struct TestAllocator;
    
    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 16; 
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    unsafe {
        let _ = table_inner.ctrl(0);
    }
}

#[test]
fn test_ctrl_valid_index_mid() {
    struct TestAllocator;
    
    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 16; 
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    let mid_index = table_inner.num_ctrl_bytes() / 2;
    
    unsafe {
        let _ = table_inner.ctrl(mid_index);
    }
}

#[test]
fn test_ctrl_valid_index_end() {
    struct TestAllocator;
    
    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 16; 
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    let end_index = table_inner.num_ctrl_bytes() - 1;
    
    unsafe {
        let _ = table_inner.ctrl(end_index);
    }
}

#[test]
#[should_panic]
fn test_ctrl_invalid_index() {
    struct TestAllocator;
    
    let alloc = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 16; 
    let table_inner = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    
    let invalid_index = table_inner.num_ctrl_bytes(); 
    
    unsafe {
        let _ = table_inner.ctrl(invalid_index); 
    }
}

