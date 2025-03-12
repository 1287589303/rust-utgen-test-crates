// Answer 0

#[test]
fn test_write_to_len_empty_table() {
    let classes = ByteClasses::empty();
    let table: Vec<u32> = vec![];

    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };

    transition_table.write_to_len();
}

#[test]
fn test_write_to_len_single_element_table() {
    let mut classes = ByteClasses::empty();
    classes.set(0, 1);
    let table: Vec<u32> = vec![0];

    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };

    transition_table.write_to_len();
}

#[test]
fn test_write_to_len_full_table() {
    let mut classes = ByteClasses::empty();
    for i in 0..256 {
        classes.set(i as u8, i as u8);
    }
    let table: Vec<u32> = (0..256).collect();

    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 8,
    };

    transition_table.write_to_len();
}

#[test]
fn test_write_to_len_edge_case() {
    let mut classes = ByteClasses::singletons();
    let table: Vec<u32> = vec![0; 256];

    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 2,
    };

    transition_table.write_to_len();
}

