// Answer 0

#[test]
fn test_state_splits_reverse() {
    let targets: Vec<u32> = vec![1, 2, 3];
    let reverse = true;
    let state = State::Splits { targets, reverse };

    let mut buffer = String::new();
    let f = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = state.fmt(f);
}

#[test]
fn test_state_splits_forward() {
    let targets: Vec<u32> = vec![4, 5];
    let reverse = false;
    let state = State::Splits { targets, reverse };

    let mut buffer = String::new();
    let f = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = state.fmt(f);
}

#[test]
#[should_panic]
fn test_state_splits_with_empty_targets() {
    let targets: Vec<u32> = vec![]; 
    let reverse = false;
    let state = State::Splits { targets, reverse };

    let mut buffer = String::new();
    let f = &mut core::fmt::Formatter::new(&mut buffer);
    let _ = state.fmt(f);
}

