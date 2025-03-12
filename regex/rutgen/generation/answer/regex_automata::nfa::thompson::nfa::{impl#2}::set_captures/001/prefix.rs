// Answer 0

#[test]
fn test_set_captures_with_empty_inner_vector() {
    let mut inner = Inner::default();
    let captures: Vec<Vec<Option<Arc<str>>>> = vec![vec![None, Some(Arc::new("group1".to_string()))], vec![]];
    let _ = inner.set_captures(&captures);
}

#[test]
fn test_set_captures_with_exceeding_groups() {
    let mut inner = Inner::default();
    let captures: Vec<Vec<Option<Arc<str>>>> = vec![
        vec![Some(Arc::new("group1".to_string())), Some(Arc::new("group2".to_string()))], 
        vec![Some(Arc::new("group3".to_string())), Some(Arc::new("group4".to_string())), Some(Arc::new("group5".to_string()))]
    ];
    let _ = inner.set_captures(&captures);
}

