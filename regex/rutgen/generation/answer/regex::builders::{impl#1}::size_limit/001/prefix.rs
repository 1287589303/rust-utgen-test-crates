// Answer 0

#[test]
fn test_size_limit_zero() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(0);
}

#[test]
fn test_size_limit_one() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(1);
}

#[test]
fn test_size_limit_ten() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(10);
}

#[test]
fn test_size_limit_hundred() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(100);
}

#[test]
fn test_size_limit_thousand() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(1_000);
}

#[test]
fn test_size_limit_ten_thousand() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(10_000);
}

#[test]
fn test_size_limit_max() {
    let mut builder = Builder {
        pats: vec![],
        metac: meta::Config::default(),
        syntaxc: syntax::Config::default(),
    };
    builder.size_limit(usize::MAX);
}

