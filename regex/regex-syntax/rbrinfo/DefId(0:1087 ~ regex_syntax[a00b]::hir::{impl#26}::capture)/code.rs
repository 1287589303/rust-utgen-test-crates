fn capture(capture: &Capture) -> Properties {
        let p = capture.sub.properties();
        Properties(Box::new(PropertiesI {
            explicit_captures_len: p.explicit_captures_len().saturating_add(1),
            static_explicit_captures_len: p
                .static_explicit_captures_len()
                .map(|len| len.saturating_add(1)),
            literal: false,
            alternation_literal: false,
            ..*p.0.clone()
        }))
    }