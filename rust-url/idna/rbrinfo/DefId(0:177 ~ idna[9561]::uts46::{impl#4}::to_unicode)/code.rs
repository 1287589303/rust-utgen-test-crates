pub fn to_unicode<'a>(
        &self,
        domain_name: &'a [u8],
        ascii_deny_list: AsciiDenyList,
        hyphens: Hyphens,
    ) -> (Cow<'a, str>, Result<(), crate::Errors>) {
        self.to_user_interface(domain_name, ascii_deny_list, hyphens, |_, _, _| true)
    }