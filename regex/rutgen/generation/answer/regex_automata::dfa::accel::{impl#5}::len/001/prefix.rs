// Answer 0

#[test]
fn test_len_zero() {
    let accel = Accel { bytes: [0; ACCEL_CAP] };
    let _ = accel.len();
}

#[test]
fn test_len_one() {
    let accel = Accel { bytes: [1, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_two() {
    let accel = Accel { bytes: [2, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_three() {
    let accel = Accel { bytes: [3, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_four() {
    let accel = Accel { bytes: [4, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_five() {
    let accel = Accel { bytes: [5, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_six() {
    let accel = Accel { bytes: [6, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_seven() {
    let accel = Accel { bytes: [7, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

#[test]
fn test_len_eight() {
    let accel = Accel { bytes: [8, 0, 0, 0, 0, 0, 0, 0] };
    let _ = accel.len();
}

