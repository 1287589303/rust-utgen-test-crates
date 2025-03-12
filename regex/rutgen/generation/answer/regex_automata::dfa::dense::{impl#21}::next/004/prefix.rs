// Answer 0

#[test]
fn test_next_case_stride() {
    let stride = 4;
    let table = vec![StateID::default(); stride * 3]; // Need length greater than 2 * stride
    let start_map = StartByteMap::default();
    let kind = StartKind::Both;
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let mut iter = StartStateIter {
        st: start_table,
        i: stride, // This will trigger the case where `i == self.st.stride`
    };

    let result = iter.next();
}

#[test]
fn test_next_case_double_stride() {
    let stride = 4;
    let table = vec![StateID::default(); stride * 3]; // Need length greater than 2 * stride
    let start_map = StartByteMap::default();
    let kind = StartKind::Both;
    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let mut iter = StartStateIter {
        st: start_table,
        i: 2 * stride, // This will trigger the case where `i == (2 * self.st.stride)`
    };

    let result = iter.next();
}

