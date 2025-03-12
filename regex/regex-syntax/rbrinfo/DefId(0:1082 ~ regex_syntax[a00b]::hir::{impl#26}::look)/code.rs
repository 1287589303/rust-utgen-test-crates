fn look(look: Look) -> Properties {
        let inner = PropertiesI {
            minimum_len: Some(0),
            maximum_len: Some(0),
            look_set: LookSet::singleton(look),
            look_set_prefix: LookSet::singleton(look),
            look_set_suffix: LookSet::singleton(look),
            look_set_prefix_any: LookSet::singleton(look),
            look_set_suffix_any: LookSet::singleton(look),
            // This requires a little explanation. Basically, we don't consider
            // matching an empty string to be equivalent to matching invalid
            // UTF-8, even though technically matching every empty string will
            // split the UTF-8 encoding of a single codepoint when treating a
            // UTF-8 encoded string as a sequence of bytes. Our defense here is
            // that in such a case, a codepoint should logically be treated as
            // the fundamental atom for matching, and thus the only valid match
            // points are between codepoints and not bytes.
            //
            // More practically, this is true here because it's also true
            // for 'Hir::empty()', otherwise something like 'a*' would be
            // considered to match invalid UTF-8. That in turn makes this
            // property borderline useless.
            utf8: true,
            explicit_captures_len: 0,
            static_explicit_captures_len: Some(0),
            literal: false,
            alternation_literal: false,
        };
        Properties(Box::new(inner))
    }