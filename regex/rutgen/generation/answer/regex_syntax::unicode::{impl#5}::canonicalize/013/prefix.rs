// Answer 0

#[test]
fn test_canonicalize_one_letter_binary() {
    let query = ClassQuery::Binary("cf");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_valid_general_category() {
    let query = ClassQuery::Binary("General_Category");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_valid_script() {
    let query = ClassQuery::Binary("Script");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_invalid_property_name() {
    let query = ClassQuery::Binary("Invalid_Property_Name");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_lower_case_property_names() {
    let query = ClassQuery::Binary("lc");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_non_existent_property() {
    let query = ClassQuery::Binary("Non_Existent");
    let _ = query.canonicalize();
}

