// Answer 0

#[test]
fn test_get_many_key_value_mut_unique_keys() {
    let mut libraries = crate::HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
    libraries.insert("Library of Congress".to_string(), 1800);

    let got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "Herzogin-Anna-Amalia-Bibliothek",
    ]);
}

#[test]
fn test_get_many_key_value_mut_missing_key() {
    let mut libraries = crate::HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);

    let got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "Non-existent Library",
    ]);
}

#[should_panic]
#[test]
fn test_get_many_key_value_mut_duplicate_keys() {
    let mut libraries = crate::HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);

    let got = libraries.get_many_key_value_mut([
        "Bodleian Library",
        "Herzogin-Anna-Amalia-Bibliothek",
        "Herzogin-Anna-Amalia-Bibliothek",
    ]);
}

