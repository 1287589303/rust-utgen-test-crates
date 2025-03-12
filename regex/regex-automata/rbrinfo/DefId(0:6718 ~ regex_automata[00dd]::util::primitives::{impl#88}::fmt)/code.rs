fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.as_u32()).finish()
            }