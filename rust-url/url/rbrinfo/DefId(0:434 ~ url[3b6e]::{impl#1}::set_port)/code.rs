pub fn set_port(&mut self, mut port: Option<u16>) -> Result<(), ()> {
        // has_host implies !cannot_be_a_base
        if !self.has_host() || self.host() == Some(Host::Domain("")) || self.scheme() == "file" {
            return Err(());
        }
        if port.is_some() && port == parser::default_port(self.scheme()) {
            port = None
        }
        self.set_port_internal(port);
        Ok(())
    }