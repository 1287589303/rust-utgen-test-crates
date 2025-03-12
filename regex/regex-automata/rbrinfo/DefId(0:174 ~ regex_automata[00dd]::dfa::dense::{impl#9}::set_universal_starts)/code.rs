fn set_universal_starts(&mut self) {
        assert_eq!(6, Start::len(), "expected 6 start configurations");

        let start_id = |dfa: &mut OwnedDFA,
                        anchored: Anchored,
                        start: Start| {
            // This OK because we only call 'start' under conditions
            // in which we know it will succeed.
            dfa.st.start(anchored, start).expect("valid Input configuration")
        };
        if self.start_kind().has_unanchored() {
            let anchor = Anchored::No;
            let sid = start_id(self, anchor, Start::NonWordByte);
            if sid == start_id(self, anchor, Start::WordByte)
                && sid == start_id(self, anchor, Start::Text)
                && sid == start_id(self, anchor, Start::LineLF)
                && sid == start_id(self, anchor, Start::LineCR)
                && sid == start_id(self, anchor, Start::CustomLineTerminator)
            {
                self.st.universal_start_unanchored = Some(sid);
            }
        }
        if self.start_kind().has_anchored() {
            let anchor = Anchored::Yes;
            let sid = start_id(self, anchor, Start::NonWordByte);
            if sid == start_id(self, anchor, Start::WordByte)
                && sid == start_id(self, anchor, Start::Text)
                && sid == start_id(self, anchor, Start::LineLF)
                && sid == start_id(self, anchor, Start::LineCR)
                && sid == start_id(self, anchor, Start::CustomLineTerminator)
            {
                self.st.universal_start_anchored = Some(sid);
            }
        }
    }