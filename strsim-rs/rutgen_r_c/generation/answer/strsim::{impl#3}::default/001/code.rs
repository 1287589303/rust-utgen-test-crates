// Answer 0

#[test]
fn test_default_row_id() {
    let row_id = RowId::default();
    assert_eq!(row_id, RowId { val: -1 });
}

#[test]
fn test_default_row_id_val() {
    let row_id = RowId::default();
    assert_eq!(row_id.val, -1);
}

