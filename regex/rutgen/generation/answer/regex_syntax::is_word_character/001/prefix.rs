// Answer 0

#[test]
fn test_unicode_word_characters() {
    let test_cases = vec![
        'a',  // alphabetic
        'Z',  // alphabetic
        'あ', // alphabetic (Japanese)
        '\u{200C}', // join control
        '\u{200D}', // join control
        '1',  // decimal number
        '2',  // decimal number
        '\u{0661}', // Arabic-Indic digit
        '\u{0300}', // mark
        '_',  // connector punctuation
        '•',  // connector punctuation
    ];
    
    for c in test_cases {
        let _ = is_word_character(c);
    }
}

#[test]
fn test_non_word_characters() {
    let non_word_cases = vec![
        '!',  // punctuation
        '?',  // punctuation
        '@',  // punctuation
        ' ',  // whitespace
        '\t', // whitespace
        '\n', // control character
        '\u{202E}', // right-to-left embedding
    ];

    for c in non_word_cases {
        let _ = is_word_character(c);
    }
}

