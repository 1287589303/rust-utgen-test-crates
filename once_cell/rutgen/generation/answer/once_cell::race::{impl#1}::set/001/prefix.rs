// Answer 0

#[test]
fn test_set_true_empty() {
    let once_bool = OnceBool::new();
    let result = once_bool.set(true);
}

#[test]
fn test_set_false_empty() {
    let once_bool = OnceBool::new();
    let result = once_bool.set(false);
}

#[test]
fn test_set_true_full() {
    let once_bool = OnceBool::new();
    let _ = once_bool.set(true);
    let result = once_bool.set(true);
}

#[test]
fn test_set_false_full() {
    let once_bool = OnceBool::new();
    let _ = once_bool.set(false);
    let result = once_bool.set(false);
}

