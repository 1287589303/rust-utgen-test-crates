pub fn set_ip_host(&mut self, address: IpAddr) -> Result<(), ()> {
        if self.cannot_be_a_base() {
            return Err(());
        }

        let address = match address {
            IpAddr::V4(address) => Host::Ipv4(address),
            IpAddr::V6(address) => Host::Ipv6(address),
        };
        self.set_host_internal(address, None);
        Ok(())
    }