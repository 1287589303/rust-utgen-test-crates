pub fn new() -> Extractor {
        Extractor {
            kind: ExtractKind::Prefix,
            limit_class: 10,
            limit_repeat: 10,
            limit_literal_len: 100,
            limit_total: 250,
        }
    }