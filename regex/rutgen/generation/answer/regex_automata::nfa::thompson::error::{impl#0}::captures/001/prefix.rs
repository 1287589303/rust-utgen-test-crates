// Answer 0

#[test]
fn test_captures_zero_groups() {
    let err = captures::GroupInfoError { /* Initialize with zero groups */ };
    let result = BuildError::captures(err);
}

#[test]
fn test_captures_maximum_groups() {
    let err = captures::GroupInfoError { /* Initialize with maximum allowable groups */ };
    let result = BuildError::captures(err);
}

#[test]
fn test_captures_invalid_group_name() {
    let err = captures::GroupInfoError { /* Initialize with invalid group name */ };
    let result = BuildError::captures(err);
}

#[test]
fn test_captures_duplicate_group_names() {
    let err = captures::GroupInfoError { /* Initialize with duplicate group names */ };
    let result = BuildError::captures(err);
}

#[test]
fn test_captures_missing_groups() {
    let err = captures::GroupInfoError { /* Initialize with missing groups */ };
    let result = BuildError::captures(err);
}

