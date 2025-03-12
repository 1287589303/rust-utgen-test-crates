fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Utf8Sequence::*;
        match *self {
            One(ref r) => write!(f, "{:?}", r),
            Two(ref r) => write!(f, "{:?}{:?}", r[0], r[1]),
            Three(ref r) => write!(f, "{:?}{:?}{:?}", r[0], r[1], r[2]),
            Four(ref r) => {
                write!(f, "{:?}{:?}{:?}{:?}", r[0], r[1], r[2], r[3])
            }
        }
    }