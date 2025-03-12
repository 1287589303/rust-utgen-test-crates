// Answer 0

#[test]
fn test_nest_limit_zero() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.nest_limit(0);
}

#[test]
fn test_nest_limit_one() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.nest_limit(1);
}

#[test]
fn test_nest_limit_ten() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.nest_limit(10);
}

#[test]
fn test_nest_limit_hundred() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.nest_limit(100);
}

#[test]
fn test_nest_limit_max() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.nest_limit(u32::MAX);
}

