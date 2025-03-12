pub fn as_ref(&self) -> DFA<&'_ [u32]> {
        DFA {
            tt: self.tt.as_ref(),
            st: self.st.as_ref(),
            ms: self.ms.as_ref(),
            special: self.special,
            accels: self.accels(),
            pre: self.pre.clone(),
            quitset: self.quitset,
            flags: self.flags,
        }
    }