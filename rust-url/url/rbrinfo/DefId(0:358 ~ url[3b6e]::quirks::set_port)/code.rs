pub fn set_port(url: &mut Url, new_port: &str) -> Result<(), ()> {
    let result;
    {
        // has_host implies !cannot_be_a_base
        let scheme = url.scheme();
        if !url.has_host() || url.host() == Some(Host::Domain("")) || scheme == "file" {
            return Err(());
        }
        result = Parser::parse_port(
            Input::new_no_trim(new_port),
            || default_port(scheme),
            Context::Setter,
        )
    }
    if let Ok((new_port, _remaining)) = result {
        url.set_port_internal(new_port);
        Ok(())
    } else {
        Err(())
    }
}