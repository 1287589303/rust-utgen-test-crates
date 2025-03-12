fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Key<'a>(usize, Option<&'a str>);

        impl<'a> core::fmt::Debug for Key<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.0)?;
                if let Some(name) = self.1 {
                    write!(f, "/{:?}", name)?;
                }
                Ok(())
            }
        }

        let mut map = f.debug_map();
        let names = self.caps.group_info().pattern_names(self.pid);
        for (group_index, maybe_name) in names.enumerate() {
            let key = Key(group_index, maybe_name);
            match self.caps.get_group(group_index) {
                None => map.entry(&key, &None::<()>),
                Some(span) => map.entry(&key, &span),
            };
        }
        map.finish()
    }