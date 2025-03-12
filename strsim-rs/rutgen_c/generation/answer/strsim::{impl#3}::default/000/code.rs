// Answer 0

#[test]
fn test_row_id_default() {
    let row_id = RowId::default();
    assert_eq!(row_id.val, -1);
}

#[test]
fn test_row_id_equality() {
    let row_id1 = RowId { val: 0 };
    let row_id2 = RowId { val: 0 };
    let row_id3 = RowId { val: 1 };
    
    assert_eq!(row_id1, row_id2);
    assert_ne!(row_id1, row_id3);
}

