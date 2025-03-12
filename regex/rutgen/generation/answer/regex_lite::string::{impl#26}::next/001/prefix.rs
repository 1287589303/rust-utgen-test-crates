// Answer 0

#[test]
fn test_next_with_empty_iterator() {
    let options: Vec<Option<Arc<str>>> = vec![];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

#[test]
fn test_next_with_one_non_null_element() {
    let options: Vec<Option<Arc<str>>> = vec![Some(Arc::new("test".to_string()))];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

#[test]
fn test_next_with_one_null_element() {
    let options: Vec<Option<Arc<str>>> = vec![None];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

#[test]
fn test_next_with_multiple_elements_all_non_null() {
    let options: Vec<Option<Arc<str>>> = vec![
        Some(Arc::new("first".to_string())),
        Some(Arc::new("second".to_string())),
        Some(Arc::new("third".to_string())),
    ];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

#[test]
fn test_next_with_multiple_elements_with_nulls() {
    let options: Vec<Option<Arc<str>>> = vec![
        Some(Arc::new("first".to_string())),
        None,
        Some(Arc::new("third".to_string())),
    ];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

#[test]
fn test_next_with_all_null_elements() {
    let options: Vec<Option<Arc<str>>> = vec![None, None, None];
    let capture_names = CaptureNames { it: options.iter() };
    let mut iter = capture_names;
    let result = iter.next();
}

