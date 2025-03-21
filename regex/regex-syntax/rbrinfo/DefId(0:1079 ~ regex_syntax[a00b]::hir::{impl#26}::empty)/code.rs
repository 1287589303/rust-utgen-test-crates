fn empty() -> Properties {
        let inner = PropertiesI {
            minimum_len: Some(0),
            maximum_len: Some(0),
            look_set: LookSet::empty(),
            look_set_prefix: LookSet::empty(),
            look_set_suffix: LookSet::empty(),
            look_set_prefix_any: LookSet::empty(),
            look_set_suffix_any: LookSet::empty(),
            // It is debatable whether an empty regex always matches at valid
            // UTF-8 boundaries. Strictly speaking, at a byte oriented view,
            // it is clearly false. There are, for example, many empty strings
            // between the bytes encoding a '☃'.
            //
            // However, when Unicode mode is enabled, the fundamental atom
            // of matching is really a codepoint. And in that scenario, an
            // empty regex is defined to only match at valid UTF-8 boundaries
            // and to never split a codepoint. It just so happens that this
            // enforcement is somewhat tricky to do for regexes that match
            // the empty string inside regex engines themselves. It usually
            // requires some layer above the regex engine to filter out such
            // matches.
            //
            // In any case, 'true' is really the only coherent option. If it
            // were false, for example, then 'a*' would also need to be false
            // since it too can match the empty string.
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: false,
            alternation_literal: false,
        };
        Properties(Box::new(inner))
    }