// Answer 0

#[test]
fn test_drop_repetition_with_capture_non_empty() {
    let capture_sub = Hir::empty(); // Assuming empty captures
    let capture = Capture { index: 0, name: None, sub: Box::new(capture_sub) };
    let repetition_sub = Hir::capture(capture.clone());
    let repetition = Repetition { min: 1, max: Some(5), greedy: true, sub: Box::new(repetition_sub) };
    
    let mut hir = Hir { 
        kind: HirKind::Repetition(repetition), 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };
    
    // This will trigger the drop
    drop(hir);
}

#[test]
fn test_drop_repetition_with_capture_non_empty_multiple() {
    let capture_sub1 = Hir::empty(); // Assuming empty captures
    let capture1 = Capture { index: 0, name: None, sub: Box::new(capture_sub1) };
    let capture_sub2 = Hir::empty(); // Assuming another empty capture
    let capture2 = Capture { index: 1, name: Some(Box::new("second".into())), sub: Box::new(capture_sub2) };
    
    let repetition_sub = Hir::capture(capture1.clone());
    let repetition_sub2 = Hir::capture(capture2.clone());
    let repetition = Repetition { 
        min: 1, max: Some(5), greedy: true, 
        sub: Box::new(HirKind::Alternation(vec![repetition_sub, repetition_sub2])) 
    };
    
    let mut hir = Hir { 
        kind: HirKind::Repetition(repetition), 
        is_start_anchored: false, 
        is_match_empty: false, 
        static_explicit_captures_len: None 
    };

    // This will trigger the drop
    drop(hir);
}

