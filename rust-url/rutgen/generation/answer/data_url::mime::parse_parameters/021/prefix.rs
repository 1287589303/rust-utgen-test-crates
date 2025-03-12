// Answer 0

#[test]
fn test_parse_parameters_empty_name_with_quoted_value() {
    let mut parameters: Vec<(String, String)> = Vec::new();
    let input = " ; \"abc\\\"def\"; other=value";
    
    parse_parameters(input, &mut parameters);
}

#[test]
fn test_parse_parameters_empty_name_with_quoted_value_and_escape() {
    let mut parameters: Vec<(String, String)> = Vec::new();
    let input = " ; \"hello\\;world\"; key=value";
    
    parse_parameters(input, &mut parameters);
}

#[test]
fn test_parse_parameters_empty_name_with_multiple_quoted_values() {
    let mut parameters: Vec<(String, String)> = Vec::new();
    let input = " ; \"test\\\"value\"; another=test";
    
    parse_parameters(input, &mut parameters);
}

#[test]
fn test_parse_parameters_empty_name_with_quoted_value_twice() {
    let mut parameters: Vec<(String, String)> = Vec::new();
    let input = " ; \"value\\\"here\"; something=else";
    
    parse_parameters(input, &mut parameters);
}

#[test]
fn test_parse_parameters_empty_name_with_leading_whitespace() {
    let mut parameters: Vec<(String, String)> = Vec::new();
    let input = "   ; \"value\"; param=123";
    
    parse_parameters(input, &mut parameters);
}

