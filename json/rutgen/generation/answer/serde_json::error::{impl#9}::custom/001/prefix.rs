// Answer 0

#[test]
fn test_custom_empty_string() {
    let msg = "";
    let _error = Error::custom(msg);
}

#[test]
fn test_custom_single_character() {
    let msg = "A";
    let _error = Error::custom(msg);
}

#[test]
fn test_custom_special_characters() {
    let msg = "!@#$%^&*()_+";
    let _error = Error::custom(msg);
}

#[test]
fn test_custom_long_string() {
    let msg = "a".repeat(10_000);
    let _error = Error::custom(msg);
}

#[test]
fn test_custom_varied_length_strings() {
    let messages = [
        "",                          // empty string
        "B",                        // single character
        "Hello, World!",           // normal string
        "Special chars: !@#%^&*",  // string with special characters
        "C".repeat(1_000),         // string of 1000 characters
    ];
    for msg in messages.iter() {
        let _error = Error::custom(msg);
    }
}

