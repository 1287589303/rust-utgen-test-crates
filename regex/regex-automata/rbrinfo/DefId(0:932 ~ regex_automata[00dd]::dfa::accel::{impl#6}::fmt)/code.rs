fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Accel(")?;
        let mut set = f.debug_set();
        for &b in self.needles() {
            set.entry(&crate::util::escape::DebugByte(b));
        }
        set.finish()?;
        write!(f, ")")
    }