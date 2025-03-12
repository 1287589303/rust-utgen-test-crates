fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:?} => {}",
            crate::util::escape::DebugByte(self.byte),
            self.next.as_usize()
        )
    }