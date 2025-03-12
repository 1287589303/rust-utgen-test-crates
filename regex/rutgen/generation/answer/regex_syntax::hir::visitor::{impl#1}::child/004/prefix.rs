// Answer 0

#[test]
fn test_child_repetition_min_zero_max_none() {
    let sub_hir = Hir { 
        kind: HirKind::SomeKind, 
        props: Properties::default() 
    };
    let repetition = Repetition { 
        min: 0, 
        max: None, 
        greedy: true, 
        sub: Box::new(sub_hir),
    };
    let frame = Frame::Repetition(&repetition);
    let _child = frame.child();
}

#[test]
fn test_child_repetition_min_one_max_ten() {
    let sub_hir = Hir { 
        kind: HirKind::SomeKind, 
        props: Properties::default() 
    };
    let repetition = Repetition { 
        min: 1, 
        max: Some(10), 
        greedy: false, 
        sub: Box::new(sub_hir),
    };
    let frame = Frame::Repetition(&repetition);
    let _child = frame.child();
}

#[test]
fn test_child_repetition_min_ten_max_ten() {
    let sub_hir = Hir { 
        kind: HirKind::SomeKind, 
        props: Properties::default() 
    };
    let repetition = Repetition { 
        min: 10, 
        max: Some(10), 
        greedy: true, 
        sub: Box::new(sub_hir),
    };
    let frame = Frame::Repetition(&repetition);
    let _child = frame.child();
}

#[test]
fn test_child_repetition_min_zero_max_ten() {
    let sub_hir = Hir { 
        kind: HirKind::SomeKind, 
        props: Properties::default() 
    };
    let repetition = Repetition { 
        min: 0, 
        max: Some(10), 
        greedy: false, 
        sub: Box::new(sub_hir),
    };
    let frame = Frame::Repetition(&repetition);
    let _child = frame.child();
}

#[test]
fn test_child_repetition_min_ten_max_none() {
    let sub_hir = Hir { 
        kind: HirKind::SomeKind, 
        props: Properties::default() 
    };
    let repetition = Repetition { 
        min: 10, 
        max: None, 
        greedy: true, 
        sub: Box::new(sub_hir),
    };
    let frame = Frame::Repetition(&repetition);
    let _child = frame.child();
}

