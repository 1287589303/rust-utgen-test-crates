// Answer 0

#[test]
fn test_canonicalize_one_letter() {
    let query = ClassQuery::OneLetter('A');
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_binary() {
    let query = ClassQuery::Binary("sc");
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_general_category() {
    let query = ClassQuery::ByValue {
        property_name: "General_Category",
        property_value: "Lu",
    };
    let _ = query.canonicalize();
}

#[test]
fn test_canonicalize_by_value_script() {
    let query = ClassQuery::ByValue {
        property_name: "Script",
        property_value: "Latin",
    };
    let _ = query.canonicalize();
}

