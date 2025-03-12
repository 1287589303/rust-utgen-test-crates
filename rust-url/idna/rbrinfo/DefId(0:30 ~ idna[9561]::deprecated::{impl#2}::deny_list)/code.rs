fn deny_list(&self) -> AsciiDenyList {
        if self.use_std3_ascii_rules {
            AsciiDenyList::STD3
        } else {
            AsciiDenyList::EMPTY
        }
    }