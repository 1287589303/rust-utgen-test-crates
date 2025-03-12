// Answer 0

#[test]
fn test_get_mut_on_empty_once_cell() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    let result = cell.get_mut();
}

#[test]
fn test_get_mut_after_setting_value() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    let value_mut = cell.get_mut().unwrap();
}

#[test]
fn test_mutate_value_with_get_mut() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    {
        let value_mut = cell.get_mut().unwrap();
        *value_mut = 100;
    }
    let value_after_mutation = cell.get().unwrap();
}

#[test]
fn test_multiple_get_mut_calls() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    {
        let value_mut1 = cell.get_mut().unwrap();
        *value_mut1 = 100;
        
        let value_mut2 = cell.get_mut().unwrap();
        *value_mut2 += 50;
    }
    let value_after_double_mutation = cell.get().unwrap();
}

