fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Accels(")?;
        let mut list = f.debug_list();
        for a in self.iter() {
            list.entry(&a);
        }
        list.finish()?;
        write!(f, ")")
    }