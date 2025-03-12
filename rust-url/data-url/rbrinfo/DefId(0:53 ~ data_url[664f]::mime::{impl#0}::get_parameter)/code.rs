pub fn get_parameter<P>(&self, name: &P) -> Option<&str>
    where
        P: ?Sized + PartialEq<str>,
    {
        self.parameters
            .iter()
            .find(|&(n, _)| name == &**n)
            .map(|(_, v)| &**v)
    }