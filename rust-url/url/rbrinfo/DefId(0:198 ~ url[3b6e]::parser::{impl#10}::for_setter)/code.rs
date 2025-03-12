pub fn for_setter(serialization: String) -> Parser<'a> {
        Parser {
            serialization,
            base_url: None,
            query_encoding_override: None,
            violation_fn: None,
            context: Context::Setter,
        }
    }