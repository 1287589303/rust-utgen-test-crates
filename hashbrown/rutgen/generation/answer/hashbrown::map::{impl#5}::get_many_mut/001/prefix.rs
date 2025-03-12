// Answer 0

#[test]
fn test_get_many_mut_single_existing() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);

    let [Some(a)] = libraries.get_many_mut(["Bodleian Library"]) else { panic!() };
}

#[test]
fn test_get_many_mut_multiple_existing() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    let [Some(a), Some(b)] = libraries.get_many_mut(["Bodleian Library", "Athenæum"]) else { panic!() };
}

#[test]
fn test_get_many_mut_one_missing() {
    let mut libraries = HashMap::new();
    libraries.insert("Bodleian Library".to_string(), 1602);
    libraries.insert("Athenæum".to_string(), 1807);

    let got = libraries.get_many_mut(["Bodleian Library", "New York Public Library"]);
}

#[test]
#[should_panic]
fn test_get_many_mut_duplicate_keys() {
    let mut libraries = HashMap::new();
    libraries.insert("Athenæum".to_string(), 1807);

    let _ = libraries.get_many_mut(["Athenæum", "Athenæum"]);
}

