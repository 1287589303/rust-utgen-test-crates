// Answer 0

#[test]
fn test_write_to_success_with_empty_table() {
    use crate::util::alphabet::ByteClasses;

    let classes = ByteClasses::empty();
    let stride2 = 1;
    let table: Vec<u32> = vec![];

    let transition_table = TransitionTable {
        table,
        classes,
        stride2,
    };

    let required_len = transition_table.write_to_len();
    let mut buffer = vec![0u8; required_len];

    let result = transition_table.write_to::<crate::util::Endian>(buffer.as_mut());

    assert_eq!(result, Ok(required_len));
}

#[test]
fn test_write_to_success_with_single_byte_class() {
    use crate::util::alphabet::ByteClasses;

    let mut classes = ByteClasses::singletons();
    let stride2 = 1;
    let table: Vec<u32> = vec![];

    let transition_table = TransitionTable {
        table,
        classes,
        stride2,
    };

    let required_len = transition_table.write_to_len();
    let mut buffer = vec![0u8; required_len];

    let result = transition_table.write_to::<crate::util::Endian>(buffer.as_mut());

    assert_eq!(result, Ok(required_len));
}

