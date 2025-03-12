// Answer 0

#[test]
fn test_check_hyphens_fail_fast_true_first_not_hyphen() {
    let mut had_errors = false;
    let mut label: Vec<char> = vec!['a'];
    let result = check_hyphens(&mut label, true, true, &mut had_errors);
}

#[test]
fn test_check_hyphens_fail_fast_true_first_not_hyphen_large_label() {
    let mut had_errors = false;
    let mut label: Vec<char> = vec!['a'; 2000];
    let result = check_hyphens(&mut label, true, true, &mut had_errors);
}

#[test]
fn test_check_hyphens_fail_fast_true_multiple_characters() {
    let mut had_errors = false;
    let mut label: Vec<char> = vec!['a', 'b', 'c'];
    let result = check_hyphens(&mut label, true, true, &mut had_errors);
}

