// Answer 0

#[test]
fn test_next_empty_slice() {
    let slice: &[Option<Arc<str>>] = &[];
    let mut capture_names = CaptureNames { it: slice.iter() };
    let _ = capture_names.next();
}

#[test]
fn test_next_all_none() {
    let slice: &[Option<Arc<str>>] = &[None, None, None];
    let mut capture_names = CaptureNames { it: slice.iter() };
    let _ = capture_names.next();
    let _ = capture_names.next();
    let _ = capture_names.next();
}

#[test]
fn test_next_mixed_some_none() {
    let slice: &[Option<Arc<str>>] = &[Some(Arc::new("first".to_string())), None, Some(Arc::new("second".to_string()))];
    let mut capture_names = CaptureNames { it: slice.iter() };
    let _ = capture_names.next();
    let _ = capture_names.next();
    let _ = capture_names.next();
}

#[test]
fn test_next_all_some() {
    let slice: &[Option<Arc<str>>] = &[Some(Arc::new("one".to_string())), Some(Arc::new("two".to_string()))];
    let mut capture_names = CaptureNames { it: slice.iter() };
    let _ = capture_names.next();
    let _ = capture_names.next();
}

#[test]
fn test_next_large_slice() {
    let slice: &[Option<Arc<str>>] = &[
        Some(Arc::new("first".to_string())),
        Some(Arc::new("second".to_string())),
        None,
        Some(Arc::new("third".to_string())),
        None,
        Some(Arc::new("fourth".to_string())),
        Some(Arc::new("fifth".to_string())),
        None,
        Some(Arc::new("sixth".to_string())),
        Some(Arc::new("seventh".to_string()))
    ];
    let mut capture_names = CaptureNames { it: slice.iter() };
    for _ in 0..slice.len() {
        let _ = capture_names.next();
    }
}

