pub fn as_ref<'a>(&'a self) -> DFA<&'a [u8]> {
        DFA {
            tt: self.tt.as_ref(),
            st: self.st.as_ref(),
            special: self.special,
            pre: self.pre.clone(),
            quitset: self.quitset,
            flags: self.flags,
        }
    }