// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_false() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.ignore_whitespace(false);
}

