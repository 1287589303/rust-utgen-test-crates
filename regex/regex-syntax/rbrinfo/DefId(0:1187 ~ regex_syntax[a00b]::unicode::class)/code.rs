pub fn class(query: ClassQuery<'_>) -> Result<hir::ClassUnicode, Error> {
    use self::CanonicalClassQuery::*;

    match query.canonicalize()? {
        Binary(name) => bool_property(name),
        GeneralCategory(name) => gencat(name),
        Script(name) => script(name),
        ByValue { property_name: "Age", property_value } => {
            let mut class = hir::ClassUnicode::empty();
            for set in ages(property_value)? {
                class.union(&hir_class(set));
            }
            Ok(class)
        }
        ByValue { property_name: "Script_Extensions", property_value } => {
            script_extension(property_value)
        }
        ByValue {
            property_name: "Grapheme_Cluster_Break",
            property_value,
        } => gcb(property_value),
        ByValue { property_name: "Sentence_Break", property_value } => {
            sb(property_value)
        }
        ByValue { property_name: "Word_Break", property_value } => {
            wb(property_value)
        }
        _ => {
            // What else should we support?
            Err(Error::PropertyNotFound)
        }
    }
}