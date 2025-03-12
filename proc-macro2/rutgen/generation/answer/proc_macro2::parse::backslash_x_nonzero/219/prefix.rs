// Answer 0

#[test]
fn test_backslash_x_nonzero_err_case_zero_zero() {
    let input: Vec<(usize, char)> = vec![(0, '0'), (1, '0')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

#[test]
fn test_backslash_x_nonzero_err_case_zero_one() {
    let input: Vec<(usize, char)> = vec![(0, '0'), (1, '1')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

#[test]
fn test_backslash_x_nonzero_err_case_non_zero_zero() {
    let input: Vec<(usize, char)> = vec![(0, '1'), (1, '0')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

#[test]
fn test_backslash_x_nonzero_success_case_one_one() {
    let input: Vec<(usize, char)> = vec![(0, '1'), (1, '1')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

#[test]
fn test_backslash_x_nonzero_success_case_a_f() {
    let input: Vec<(usize, char)> = vec![(0, 'a'), (1, 'f')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

#[test]
fn test_backslash_x_nonzero_success_case_A_F() {
    let input: Vec<(usize, char)> = vec![(0, 'A'), (1, 'F')];
    let mut chars = input.iter().cloned();
    let result = backslash_x_nonzero(&mut chars);
}

