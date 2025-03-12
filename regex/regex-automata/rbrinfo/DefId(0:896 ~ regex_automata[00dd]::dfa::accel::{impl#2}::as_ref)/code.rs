pub fn as_ref(&self) -> Accels<&[AccelTy]> {
        Accels { accels: self.accels.as_ref() }
    }