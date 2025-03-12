use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::fmt::{self, Write};
use core::str::FromStr;
#[rustfmt::skip]
static IS_HTTP_TOKEN: [bool; 256] = byte_map![
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0,
];
fn parse_parameters(s: &str, parameters: &mut Vec<(String, String)>) {
    let mut semicolon_separated = s.split(';');
    while let Some(piece) = semicolon_separated.next() {
        let piece = piece.trim_start_matches(http_whitespace);
        let (name, value) = split2(piece, '=');
        let name_valid = !name.is_empty() && only_http_token_code_points(name)
            && !contains(parameters, name);
        if let Some(value) = value {
            let value = if let Some(stripped) = value.strip_prefix('"') {
                let max_len = stripped.len().saturating_sub(1);
                let mut unescaped_value = String::with_capacity(max_len);
                let mut chars = stripped.chars();
                'until_closing_quote: loop {
                    while let Some(c) = chars.next() {
                        match c {
                            '"' => break 'until_closing_quote,
                            '\\' => {
                                unescaped_value
                                    .push(
                                        chars
                                            .next()
                                            .unwrap_or_else(|| {
                                                semicolon_separated
                                                    .next()
                                                    .map(|piece| {
                                                        chars = piece.chars();
                                                        ';'
                                                    })
                                                    .unwrap_or('\\')
                                            }),
                                    )
                            }
                            _ => unescaped_value.push(c),
                        }
                    }
                    if let Some(piece) = semicolon_separated.next() {
                        unescaped_value.push(';');
                        chars = piece.chars();
                    } else {
                        break;
                    }
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                unescaped_value
            } else {
                let value = value.trim_end_matches(http_whitespace);
                if value.is_empty() {
                    continue;
                }
                if !name_valid || !valid_value(value) {
                    continue;
                }
                value.to_owned()
            };
            parameters.push((name.to_ascii_lowercase(), value))
        }
    }
}
fn split2(s: &str, separator: char) -> (&str, Option<&str>) {
    let mut iter = s.splitn(2, separator);
    let first = iter.next().unwrap();
    (first, iter.next())
}
fn contains(parameters: &[(String, String)], name: &str) -> bool {
    parameters.iter().any(|(n, _)| n == name)
}
fn only_http_token_code_points(s: &str) -> bool {
    s.bytes().all(|byte| IS_HTTP_TOKEN[byte as usize])
}
fn valid_value(s: &str) -> bool {
    s.chars().all(|c| { matches!(c, '\t' | ' '..='~' | '\u{80}'..='\u{FF}') })
}
