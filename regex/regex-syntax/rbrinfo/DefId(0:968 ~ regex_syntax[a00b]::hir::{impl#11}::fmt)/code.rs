fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use crate::debug::Byte;

        let mut fmter = f.debug_set();
        match *self {
            Class::Unicode(ref cls) => {
                for r in cls.ranges().iter() {
                    fmter.entry(&(r.start..=r.end));
                }
            }
            Class::Bytes(ref cls) => {
                for r in cls.ranges().iter() {
                    fmter.entry(&(Byte(r.start)..=Byte(r.end)));
                }
            }
        }
        fmter.finish()
    }