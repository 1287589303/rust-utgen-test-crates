// Answer 0

#[test]
fn test_static_explicit_captures_len_empty_string() {
    let pattern = String::new();
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_no_captures() {
    let pattern = String::from("abc");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_with_explicit_capture() {
    let pattern = String::from("(abc)");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_with_nested_captures() {
    let pattern = String::from("((abc)(def))");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_start_anchored() {
    let pattern = String::from("^abc(\\d+)$");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_no_captures_start_anchored() {
    let pattern = String::from("^abc$");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_large_number_of_captures() {
    let pattern = String::from("(((a)(b))(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)(m)(n)(o)(p)(q)(r)(s)(t)(u)(v)(w)(x)(y)(z)))");
    let hir = hir::Hir::from_str(&pattern).unwrap();
    let nfa = NFA::new(Config::default(), pattern, &hir).unwrap();
    let _ = nfa.static_explicit_captures_len();
}

