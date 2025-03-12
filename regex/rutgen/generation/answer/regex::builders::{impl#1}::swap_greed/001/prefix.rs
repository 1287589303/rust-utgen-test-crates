// Answer 0

#[test]
fn test_swap_greed_true() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_false() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.swap_greed(false);
}

