// Answer 0

#[test]
fn test_builder_with_valid_pattern() {
    use regex_automata::{
        hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
    };

    let re = Regex::builder()
        .syntax(syntax::Config::new().utf8(true))
        .thompson(thompson::Config::new().utf8(true))
        .build(r"foo(?-u:[^b])ar.*").unwrap();

    let mut cache = re.create_cache();
    let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
    let got = re.find(&mut cache, haystack);
}

#[test]
fn test_builder_with_another_valid_pattern() {
    use regex_automata::{
        hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
    };

    let re = Regex::builder()
        .syntax(syntax::Config::new().utf8(true))
        .thompson(thompson::Config::new().utf8(true))
        .build(r"bar").unwrap();

    let mut cache = re.create_cache();
    let haystack = b"barbaz";
    let got = re.find(&mut cache, haystack);
}

#[test]
fn test_builder_with_empty_string() {
    use regex_automata::{
        hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
    };

    let re = Regex::builder()
        .syntax(syntax::Config::new().utf8(true))
        .thompson(thompson::Config::new().utf8(true))
        .build("").unwrap();

    let mut cache = re.create_cache();
    let haystack = b"";
    let got = re.find(&mut cache, haystack);
}

#[test]
fn test_builder_with_non_utf8_config() {
    use regex_automata::{
        hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
    };

    let re = Regex::builder()
        .syntax(syntax::Config::new().utf8(false))
        .thompson(thompson::Config::new().utf8(false))
        .build(r"foo(?-u:[^b])ar.*").unwrap();

    let mut cache = re.create_cache();
    let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
    let got = re.find(&mut cache, haystack);
}

#[test]
fn test_builder_with_pattern_not_matching() {
    use regex_automata::{
        hybrid::regex::Regex, nfa::thompson, util::syntax,
    };

    let re = Regex::builder()
        .syntax(syntax::Config::new().utf8(true))
        .thompson(thompson::Config::new().utf8(true))
        .build(r"[^a-z]").unwrap();

    let mut cache = re.create_cache();
    let haystack = b"123";
    let got = re.find(&mut cache, haystack);
}

