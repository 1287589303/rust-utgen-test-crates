// Answer 0

#[test]
fn test_state_valid_id_zero() {
    let stride2 = 1;
    let table = vec![StateID(0)];
    let classes = ByteClasses([0; 256]);
    let valid_id = StateID(0);
    let transition_table = TransitionTable { table, classes, stride2 };
    let state = transition_table.state(valid_id);
}

#[test]
fn test_state_valid_id_stride2_two() {
    let stride2 = 2;
    let table = vec![StateID(0), StateID(1), StateID(2)];
    let classes = ByteClasses([0; 256]);
    let valid_id = StateID(0);
    let transition_table = TransitionTable { table, classes, stride2 };
    let state = transition_table.state(valid_id);
}

#[test]
fn test_state_valid_id_stride2_four() {
    let stride2 = 3;
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5)];
    let classes = ByteClasses([0; 256]);
    let valid_id = StateID(0);
    let transition_table = TransitionTable { table, classes, stride2 };
    let state = transition_table.state(valid_id);
}

#[test]
fn test_state_valid_id_stride2_eight() {
    let stride2 = 4;
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7)];
    let classes = ByteClasses([0; 256]);
    let valid_id = StateID(0);
    let transition_table = TransitionTable { table, classes, stride2 };
    let state = transition_table.state(valid_id);
}

#[test]
fn test_state_valid_id_max_stride() {
    let stride2 = 9;
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7), StateID(8), StateID(9)];
    let classes = ByteClasses([0; 256]);
    let valid_id = StateID(0);
    let transition_table = TransitionTable { table, classes, stride2 };
    let state = transition_table.state(valid_id);
}

