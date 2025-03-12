fn from(host: Host<String>) -> HostInternal {
        match host {
            Host::Domain(ref s) if s.is_empty() => HostInternal::None,
            Host::Domain(_) => HostInternal::Domain,
            Host::Ipv4(address) => HostInternal::Ipv4(address),
            Host::Ipv6(address) => HostInternal::Ipv6(address),
        }
    }