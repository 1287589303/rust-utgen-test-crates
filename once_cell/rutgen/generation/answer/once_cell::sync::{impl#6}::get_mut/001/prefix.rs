// Answer 0

#[test]
fn test_get_mut_empty_cell() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    let _result = cell.get_mut(); // Should return None
}

#[test]
fn test_get_mut_initialized_cell() {
    let mut cell: OnceCell<u32> = OnceCell::with_value(42);
    let mut_result = cell.get_mut(); // Should return Some(&mut 42)
    let _value = mut_result.map(|v| *v += 1); // Modifying the value
}

#[test]
fn test_get_mut_after_set() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(92).unwrap();
    let mut_result = cell.get_mut(); // Should return Some(&mut 92)
    let _value = mut_result.map(|v| *v += 1); // Modifying the value
}

#[test]
fn test_get_mut_after_reset() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(84).unwrap();
    cell = OnceCell::new(); // Resetting the cell
    let _result = cell.get_mut(); // Should return None again
}

#[test]
fn test_get_mut_with_string() {
    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("Hello".to_string()).unwrap();
    let mut_result = cell.get_mut(); // Should return Some(&mut "Hello")
    let _value = mut_result.map(|s| s.push_str(", World!")); // Modifying the string
}

