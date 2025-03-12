// Answer 0

#[test]
fn test_pattern_len_mismatch() {
    use crate::hybrid::regex::Regex;
    use crate::hybrid::dfa::DFA;
    use crate::error::BuildError;

    let dfa1 = DFA::new_many(&[r"[a-z]+"]).unwrap();
    let dfa2 = DFA::new_many(&[r"\d+", r"[a-z]{2,}"]).unwrap();
    
    let re = Regex {
        forward: dfa1,
        reverse: dfa2,
    };

    let _ = re.pattern_len();
}

#[test]
fn test_pattern_len_different_lengths() {
    use crate::hybrid::regex::Regex;
    use crate::hybrid::dfa::DFA;
    use crate::error::BuildError;

    let dfa1 = DFA::new_many(&[r"[a-z]"]).unwrap();
    let dfa2 = DFA::new_many(&[r"\d+", r"[a-z]{3,}"]).unwrap();

    let re = Regex {
        forward: dfa1,
        reverse: dfa2,
    };

    let _ = re.pattern_len();
}

