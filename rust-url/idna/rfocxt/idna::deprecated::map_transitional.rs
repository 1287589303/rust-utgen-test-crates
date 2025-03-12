use alloc::borrow::Cow;
use alloc::string::String;
use crate::uts46::*;
use crate::Errors;
fn map_transitional(domain: &str, transitional: bool) -> Cow<'_, str> {
    if !transitional {
        return Cow::Borrowed(domain);
    }
    let mut chars = domain.chars();
    loop {
        let prev = chars.clone();
        if let Some(c) = chars.next() {
            match c {
                'ß' | 'ẞ' | 'ς' | '\u{200C}' | '\u{200D}' => {
                    let mut s = String::with_capacity(domain.len());
                    let tail = prev.as_str();
                    let head = &domain[..domain.len() - tail.len()];
                    s.push_str(head);
                    for c in tail.chars() {
                        match c {
                            'ß' | 'ẞ' => {
                                s.push_str("ss");
                            }
                            'ς' => {
                                s.push('σ');
                            }
                            '\u{200C}' | '\u{200D}' => {}
                            _ => {
                                s.push(c);
                            }
                        }
                    }
                    return Cow::Owned(s);
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    Cow::Borrowed(domain)
}
