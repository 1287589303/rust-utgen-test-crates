// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_limit() {
    let re = regex_lite::Regex::new(r"\s+").unwrap();  // using a regex that matches whitespace
    let hay = "Hello     World   How Are   You";  // multiple whitespace matches
    let limit = 2;  // positive limit to replace
    let rep = " ";  // a simple space replacement with no expansions

    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_no_expansion_and_boundary_limit() {
    let re = regex_lite::Regex::new(r"\s+").unwrap();  // using a regex that matches whitespace
    let hay = "Rust   is   great and   Rust   is   fast";  // multiple whitespace matches
    let limit = 2;  // positive limit for replacing
    let rep = "-";  // a simple string replacement with no expansions

    let result = re.replacen(hay, limit, rep);
} 

#[test]
fn test_replacen_non_overlapping_matches() {
    let re = regex_lite::Regex::new(r"Rust").unwrap();  // matches "Rust"
    let hay = "Rust is a systems programming language. Rust is great.";
    let limit = 1;  // limiting to one replacement
    let rep = "C++";  // a simple replacement without capture expansions

    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_large_haystack() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();  // regex that matches one or more digits
    let hay = "1 2 3 4 5 6 7 8 9 10";  // sufficient length to have multiple matches
    let limit = 5;  // limit set to 5
    let rep = "X";  // replacement with no expansions

    let result = re.replacen(hay, limit, rep);
}

