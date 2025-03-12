pub fn socket_addrs(
        &self,
        default_port_number: impl Fn() -> Option<u16>,
    ) -> io::Result<alloc::vec::Vec<SocketAddr>> {
        // Note: trying to avoid the Vec allocation by returning `impl AsRef<[SocketAddr]>`
        // causes borrowck issues because the return value borrows `default_port_number`:
        //
        // https://github.com/rust-lang/rfcs/blob/master/text/1951-expand-impl-trait.md#scoping-for-type-and-lifetime-parameters
        //
        // > This RFC proposes that *all* type parameters are considered in scope
        // > for `impl Trait` in return position

        fn io_result<T>(opt: Option<T>, message: &str) -> io::Result<T> {
            opt.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, message))
        }

        let host = io_result(self.host(), "No host name in the URL")?;
        let port = io_result(
            self.port_or_known_default().or_else(default_port_number),
            "No port number in the URL",
        )?;
        Ok(match host {
            Host::Domain(domain) => (domain, port).to_socket_addrs()?.collect(),
            Host::Ipv4(ip) => vec![(ip, port).into()],
            Host::Ipv6(ip) => vec![(ip, port).into()],
        })
    }