// Answer 0

#[test]
fn test_is_always_anchored_end_with_end() {
    let props = hir::Properties::new();
    let mut props_union = props.clone();
    props_union.set_look_set_suffix(vec![Look::End]);
    
    let regex_info = RegexInfo(Arc::new(RegexInfoI {
        config: Config::default(),
        props: vec![props],
        props_union,
    }));

    let result = regex_info.is_always_anchored_end();
}

#[test]
fn test_is_always_anchored_end_without_end() {
    let props = hir::Properties::new();
    let mut props_union = props.clone();
    props_union.set_look_set_suffix(vec![]); // No Look::End present
    
    let regex_info = RegexInfo(Arc::new(RegexInfoI {
        config: Config::default(),
        props: vec![props],
        props_union,
    }));

    let result = regex_info.is_always_anchored_end();
}

