// Answer 0

#[test]
fn test_clone_from_non_empty_to_non_empty() {
    struct CloneableStruct {
        value: i32,
    }
    
    let source = OnceCell::with_value(CloneableStruct { value: 42 });
    let mut self_cell = OnceCell::with_value(CloneableStruct { value: 21 });

    self_cell.clone_from(&source);
}

#[test]
fn test_clone_from_non_empty_to_empty() {
    struct CloneableStruct {
        value: i32,
    }
    
    let source = OnceCell::with_value(CloneableStruct { value: 42 });
    let mut self_cell = OnceCell::new();

    self_cell.clone_from(&source);
}

#[test]
fn test_clone_from_empty_to_non_empty() {
    struct CloneableStruct {
        value: i32,
    }
    
    let mut self_cell = OnceCell::with_value(CloneableStruct { value: 21 });
    let source = OnceCell::new();

    self_cell.clone_from(&source);
}

#[test]
fn test_clone_from_empty_to_empty() {
    struct CloneableStruct {
        value: i32,
    }
    
    let source = OnceCell::new();
    let mut self_cell = OnceCell::new();

    self_cell.clone_from(&source);
}

