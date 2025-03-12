// Answer 0

#[test]
fn test_crlf_yes() {
    let mut builder = Builder {
        pats: Vec::new(),
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.crlf(true);
}

#[test]
fn test_crlf_no() {
    let mut builder = Builder {
        pats: Vec::new(),
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.crlf(false);
}

