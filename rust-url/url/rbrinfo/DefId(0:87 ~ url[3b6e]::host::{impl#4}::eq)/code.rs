fn eq(&self, other: &Host<T>) -> bool {
        match (self, other) {
            (Host::Domain(a), Host::Domain(b)) => a == b,
            (Host::Ipv4(a), Host::Ipv4(b)) => a == b,
            (Host::Ipv6(a), Host::Ipv6(b)) => a == b,
            (_, _) => false,
        }
    }