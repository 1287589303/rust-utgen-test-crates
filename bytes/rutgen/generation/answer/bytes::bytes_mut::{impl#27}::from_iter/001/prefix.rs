// Answer 0

#[test]
fn test_from_iter_non_empty() {
    let input: Vec<&u8> = vec![&1, &2, &3, &4, &5];
    let _result = BytesMut::from_iter(input);
}

#[test]
fn test_from_iter_empty() {
    let input: Vec<&u8> = vec![];
    let _result = BytesMut::from_iter(input);
}

#[test]
fn test_from_iter_single_element() {
    let input: Vec<&u8> = vec![&1];
    let _result = BytesMut::from_iter(input);
}

#[test]
fn test_from_iter_repeated_values() {
    let input: Vec<&u8> = vec![&1, &1, &1, &1, &1];
    let _result = BytesMut::from_iter(input);
}

#[test]
fn test_from_iter_large_number_elements() {
    let input: Vec<&u8> = (0..20).map(|i| &i as &u8).collect();
    let _result = BytesMut::from_iter(input);
}

#[test]
#[should_panic]
fn test_from_iter_exceeds_max_capacity() {
    let input: Vec<&u8> = (0..usize::MAX).map(|i| &i as &u8).collect();
    let _result = BytesMut::from_iter(input);
}

