// Answer 0

#[test]
fn test_canonical_binary_case_sc() {
    let query = ClassQuery::Binary("sc");
    let _result = query.canonical_binary("sc");
}

#[test]
fn test_canonical_binary_case_cf() {
    let query = ClassQuery::Binary("cf");
    let _result = query.canonical_binary("cf");
}

#[test]
fn test_canonical_binary_case_lc() {
    let query = ClassQuery::Binary("lc");
    let _result = query.canonical_binary("lc");
}

