fn byte_classes_from_nfa(
        &self,
        nfa: &thompson::NFA,
        quit: &ByteSet,
    ) -> ByteClasses {
        if !self.get_byte_classes() {
            // The lazy DFA will always use the equivalence class map, but
            // enabling this option is useful for debugging. Namely, this will
            // cause all transitions to be defined over their actual bytes
            // instead of an opaque equivalence class identifier. The former is
            // much easier to grok as a human.
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            // It is important to distinguish any "quit" bytes from all other
            // bytes. Otherwise, a non-quit byte may end up in the same class
            // as a quit byte, and thus cause the DFA stop when it shouldn't.
            //
            // Test case:
            //
            //   regex-cli find match hybrid --unicode-word-boundary \
            //     -p '^#' -p '\b10\.55\.182\.100\b' -y @conn.json.1000x.log
            if !quit.is_empty() {
                set.add_set(&quit);
            }
            set.byte_classes()
        }
    }