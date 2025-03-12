// Answer 0

#[test]
fn test_get_or_try_init_initialized_case() {
    struct Test;
    let cell = OnceCell::<i32>::new();
    assert!(cell.get().is_none());
    
    let result = cell.get_or_try_init(|| Ok(42));
    let value = result.unwrap();
    assert_eq!(*value, 42);
    assert_eq!(cell.get(), Some(&42));
}

#[test]
#[should_panic]
fn test_get_or_try_init_reentrant_initialization() {
    struct Test;
    let cell = OnceCell::<i32>::new();
    
    let _ = cell.get_or_try_init(|| {
        cell.get_or_try_init(|| Ok(99)); // Attempting to reinitialize
        Ok(42)
    });
}

#[test]
fn test_get_or_try_init_empty_case() {
    struct Test;
    let cell = OnceCell::<String>::new();
    
    let result: Result<&String, ()> = cell.get_or_try_init(|| Err(()));
    assert!(result.is_err());
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_error_case() {
    struct Test;
    let cell = OnceCell::<f64>::new();
    
    let result: Result<&f64, String> = cell.get_or_try_init(|| Err("Failed".to_string()));
    assert!(result.is_err());
    assert!(cell.get().is_none());
}

#[test]
fn test_get_or_try_init_successful_initialization() {
    struct Test;
    let cell = OnceCell::<u32>::new();
    
    let result = cell.get_or_try_init(|| Ok(100));
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 100);
    assert_eq!(cell.get(), Some(&100));
}

