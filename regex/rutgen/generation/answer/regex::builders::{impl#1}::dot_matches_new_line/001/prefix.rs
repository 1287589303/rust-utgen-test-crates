// Answer 0

#[test]
fn test_dot_matches_new_line_true() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.dot_matches_new_line(true);
}

#[test]
fn test_dot_matches_new_line_false() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.dot_matches_new_line(false);
}

