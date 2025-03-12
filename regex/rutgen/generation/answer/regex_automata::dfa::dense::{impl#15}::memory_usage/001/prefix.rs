// Answer 0

#[test]
fn test_memory_usage_zero_entries() {
    let table: Vec<u32> = Vec::new();
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    let _ = transition_table.memory_usage();
}

#[test]
fn test_memory_usage_one_entry() {
    let table = vec![0u32];
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    let _ = transition_table.memory_usage();
}

#[test]
fn test_memory_usage_two_entries() {
    let table = vec![0u32, 1];
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    let _ = transition_table.memory_usage();
}

#[test]
fn test_memory_usage_maximum_entries() {
    let table: Vec<u32> = (0..257).map(|i| i as u32).collect();
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    let _ = transition_table.memory_usage();
}

#[test]
fn test_memory_usage_mid_size_entries() {
    let table: Vec<u32> = (0..256).map(|i| i as u32).collect();
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    let _ = transition_table.memory_usage();
}

