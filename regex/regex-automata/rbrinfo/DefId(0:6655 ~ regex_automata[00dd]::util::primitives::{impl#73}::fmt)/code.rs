fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(
                    f,
                    "failed to create {} from {:?}, which exceeds {:?}",
                    stringify!($name),
                    self.attempted(),
                    $name::MAX,
                )
            }