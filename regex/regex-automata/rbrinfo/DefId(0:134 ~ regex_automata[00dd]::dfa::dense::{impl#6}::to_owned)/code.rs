pub fn to_owned(&self) -> OwnedDFA {
        DFA {
            tt: self.tt.to_owned(),
            st: self.st.to_owned(),
            ms: self.ms.to_owned(),
            special: self.special,
            accels: self.accels().to_owned(),
            pre: self.pre.clone(),
            quitset: self.quitset,
            flags: self.flags,
        }
    }