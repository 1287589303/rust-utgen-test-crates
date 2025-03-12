// Answer 0

#[test]
fn test_line_terminator_zero() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.line_terminator(0);
}

#[test]
fn test_line_terminator_middle() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.line_terminator(128);
}

#[test]
fn test_line_terminator_max() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.line_terminator(255);
}

