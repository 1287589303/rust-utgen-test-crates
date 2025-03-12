pub fn to_owned(&self) -> Accels<alloc::vec::Vec<AccelTy>> {
        Accels { accels: self.accels.as_ref().to_vec() }
    }