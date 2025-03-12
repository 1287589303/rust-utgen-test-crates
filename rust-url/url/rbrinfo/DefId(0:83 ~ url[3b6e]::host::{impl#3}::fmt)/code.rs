fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Host::Domain(ref domain) => domain.as_ref().fmt(f),
            Host::Ipv4(ref addr) => addr.fmt(f),
            Host::Ipv6(ref addr) => {
                f.write_str("[")?;
                write_ipv6(addr, f)?;
                f.write_str("]")
            }
        }
    }