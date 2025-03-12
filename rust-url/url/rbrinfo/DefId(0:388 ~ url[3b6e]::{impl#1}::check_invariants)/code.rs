pub fn check_invariants(&self) -> Result<(), String> {
        macro_rules! assert {
            ($x: expr) => {
                if !$x {
                    return Err(format!(
                        "!( {} ) for URL {:?}",
                        stringify!($x),
                        self.serialization
                    ));
                }
            };
        }

        macro_rules! assert_eq {
            ($a: expr, $b: expr) => {
                {
                    let a = $a;
                    let b = $b;
                    if a != b {
                        return Err(format!("{:?} != {:?} ({} != {}) for URL {:?}",
                                           a, b, stringify!($a), stringify!($b),
                                           self.serialization))
                    }
                }
            }
        }

        assert!(self.scheme_end >= 1);
        assert!(self.byte_at(0).is_ascii_alphabetic());
        assert!(self
            .slice(1..self.scheme_end)
            .chars()
            .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '+' | '-' | '.')));
        assert_eq!(self.byte_at(self.scheme_end), b':');

        if self.slice(self.scheme_end + 1..).starts_with("//") {
            // URL with authority
            if self.username_end != self.serialization.len() as u32 {
                match self.byte_at(self.username_end) {
                    b':' => {
                        assert!(self.host_start >= self.username_end + 2);
                        assert_eq!(self.byte_at(self.host_start - 1), b'@');
                    }
                    b'@' => assert!(self.host_start == self.username_end + 1),
                    _ => assert_eq!(self.username_end, self.scheme_end + 3),
                }
            }
            assert!(self.host_start >= self.username_end);
            assert!(self.host_end >= self.host_start);
            let host_str = self.slice(self.host_start..self.host_end);
            match self.host {
                HostInternal::None => assert_eq!(host_str, ""),
                HostInternal::Ipv4(address) => assert_eq!(host_str, address.to_string()),
                HostInternal::Ipv6(address) => {
                    let h: Host<String> = Host::Ipv6(address);
                    assert_eq!(host_str, h.to_string())
                }
                HostInternal::Domain => {
                    if SchemeType::from(self.scheme()).is_special() {
                        assert!(!host_str.is_empty())
                    }
                }
            }
            if self.path_start == self.host_end {
                assert_eq!(self.port, None);
            } else {
                assert_eq!(self.byte_at(self.host_end), b':');
                let port_str = self.slice(self.host_end + 1..self.path_start);
                assert_eq!(
                    self.port,
                    Some(port_str.parse::<u16>().expect("Couldn't parse port?"))
                );
            }
            assert!(
                self.path_start as usize == self.serialization.len()
                    || matches!(self.byte_at(self.path_start), b'/' | b'#' | b'?')
            );
        } else {
            // Anarchist URL (no authority)
            assert_eq!(self.username_end, self.scheme_end + 1);
            assert_eq!(self.host_start, self.scheme_end + 1);
            assert_eq!(self.host_end, self.scheme_end + 1);
            assert_eq!(self.host, HostInternal::None);
            assert_eq!(self.port, None);
            if self.path().starts_with("//") {
                // special case when first path segment is empty
                assert_eq!(self.byte_at(self.scheme_end + 1), b'/');
                assert_eq!(self.byte_at(self.scheme_end + 2), b'.');
                assert_eq!(self.path_start, self.scheme_end + 3);
            } else {
                assert_eq!(self.path_start, self.scheme_end + 1);
            }
        }
        if let Some(start) = self.query_start {
            assert!(start >= self.path_start);
            assert_eq!(self.byte_at(start), b'?');
        }
        if let Some(start) = self.fragment_start {
            assert!(start >= self.path_start);
            assert_eq!(self.byte_at(start), b'#');
        }
        if let (Some(query_start), Some(fragment_start)) = (self.query_start, self.fragment_start) {
            assert!(fragment_start > query_start);
        }

        let other = Url::parse(self.as_str()).expect("Failed to parse myself?");
        assert_eq!(&self.serialization, &other.serialization);
        assert_eq!(self.scheme_end, other.scheme_end);
        assert_eq!(self.username_end, other.username_end);
        assert_eq!(self.host_start, other.host_start);
        assert_eq!(self.host_end, other.host_end);
        assert!(
            self.host == other.host ||
                // XXX No host round-trips to empty host.
                // See https://github.com/whatwg/url/issues/79
                (self.host_str(), other.host_str()) == (None, Some(""))
        );
        assert_eq!(self.port, other.port);
        assert_eq!(self.path_start, other.path_start);
        assert_eq!(self.query_start, other.query_start);
        assert_eq!(self.fragment_start, other.fragment_start);
        Ok(())
    }