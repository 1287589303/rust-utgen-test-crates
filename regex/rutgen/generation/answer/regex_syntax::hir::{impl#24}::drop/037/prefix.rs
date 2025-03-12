// Answer 0

#[test]
fn test_drop_with_capture_and_non_empty_sub() {
    let capture_sub = Hir {
        kind: HirKind::Class(Class {}),
        props: Properties(Box::new(PropertiesI {})),
    };
    let capture = Capture {
        index: 0,
        name: Some(Box::from("test")),
        sub: Box::new(capture_sub),
    };
    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties(Box::new(PropertiesI {})),
    };
    drop(hir_capture);
}

#[test]
fn test_drop_with_capture_and_class_as_sub() {
    let class_hir = Hir {
        kind: HirKind::class(Class {}),
        props: Properties(Box::new(PropertiesI {})),
    };
    let capture = Capture {
        index: 1,
        name: None,
        sub: Box::new(class_hir),
    };
    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties(Box::new(PropertiesI {})),
    };
    drop(hir_capture);
}

#[test]
fn test_drop_with_capture_and_look_as_sub() {
    let look_hir = Hir {
        kind: HirKind::Look(Look {}),
        props: Properties(Box::new(PropertiesI {})),
    };
    let capture = Capture {
        index: 2,
        name: Some(Box::from("capture2")),
        sub: Box::new(look_hir),
    };
    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties(Box::new(PropertiesI {})),
    };
    drop(hir_capture);
}

#[test]
fn test_drop_with_capture_and_empty_as_sub() {
    let empty_hir = Hir {
        kind: HirKind::Empty,
        props: Properties(Box::new(PropertiesI {})),
    };
    let capture = Capture {
        index: 3,
        name: None,
        sub: Box::new(empty_hir),
    };
    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties(Box::new(PropertiesI {})),
    };
    drop(hir_capture);
} 

#[test]
fn test_drop_with_capture_followed_by_class() {
    let class_hir = Hir {
        kind: HirKind::Class(Class {}),
        props: Properties(Box::new(PropertiesI {})),
    };
    let capture_sub = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: Some(Box::from("sub_capture")),
            sub: Box::new(class_hir),
        }),
        props: Properties(Box::new(PropertiesI {})),
    };
    let hir_capture = Hir {
        kind: HirKind::Capture(Capture {
            index: 0,
            name: None,
            sub: Box::new(capture_sub),
        }),
        props: Properties(Box::new(PropertiesI {})),
    };
    drop(hir_capture);
}

