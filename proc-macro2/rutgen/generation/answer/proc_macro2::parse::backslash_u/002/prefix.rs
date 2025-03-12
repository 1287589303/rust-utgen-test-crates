// Answer 0

#[test]
fn test_backslash_u_invalid_length() {
    let input = vec![
        (0, '{'), 
        (1, '0'), 
        (2, '1'), 
        (3, '2'), 
        (4, '3'), 
        (5, '4'), 
        (6, '5'), 
        (7, '6'), 
    ].into_iter();

    let result = backslash_u(&mut input);
}

#[test]
fn test_backslash_u_valid_hex_before_limit() {
    let input = vec![
        (0, '{'), 
        (1, '0'), 
        (2, '1'), 
        (3, '2'), 
        (4, '3'), 
        (5, '4'), 
        (6, '5'), 
        (7, '}'), 
    ].into_iter();

    let result = backslash_u(&mut input);
}

#[test]
fn test_backslash_u_invalid_character() {
    let input = vec![
        (0, '{'), 
        (1, '0'), 
        (2, '1'), 
        (3, '2'), 
        (4, '3'), 
        (5, '4'), 
        (6, 'G'), 
    ].into_iter();

    let result = backslash_u(&mut input);
}

