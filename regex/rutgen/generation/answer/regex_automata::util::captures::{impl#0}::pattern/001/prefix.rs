// Answer 0

#[test]
fn test_pattern_with_match() {
    use crate::util::primitives::{NonMaxUsize, SmallIndex};
    use crate::util::captures::{Captures, GroupInfo};
    use std::sync::Arc;

    let group_info = GroupInfo(Arc::new(Default::default()));
    let pattern_id = PatternID(SmallIndex::new(0));
    let captures = Captures {
        group_info,
        pid: Some(pattern_id),
        slots: vec![Some(NonMaxUsize::new(0).unwrap())],
    };
    
    let result = captures.pattern();
}

#[test]
fn test_pattern_without_match() {
    use crate::util::primitives::{NonMaxUsize, SmallIndex};
    use crate::util::captures::{Captures, GroupInfo};
    use std::sync::Arc;

    let group_info = GroupInfo(Arc::new(Default::default()));
    let captures = Captures {
        group_info,
        pid: None,
        slots: vec![None],
    };
    
    let result = captures.pattern();
}

