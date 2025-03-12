pub fn to_owned(&self) -> DFA<alloc::vec::Vec<u8>> {
        DFA {
            tt: self.tt.to_owned(),
            st: self.st.to_owned(),
            special: self.special,
            pre: self.pre.clone(),
            quitset: self.quitset,
            flags: self.flags,
        }
    }