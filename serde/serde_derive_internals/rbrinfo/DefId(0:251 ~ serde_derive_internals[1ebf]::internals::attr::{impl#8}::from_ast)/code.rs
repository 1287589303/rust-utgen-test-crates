pub fn from_ast(
        cx: &Ctxt,
        index: usize,
        field: &syn::Field,
        attrs: Option<&Variant>,
        container_default: &Default,
    ) -> Self {
        let mut ser_name = Attr::none(cx, RENAME);
        let mut de_name = Attr::none(cx, RENAME);
        let mut de_aliases = VecAttr::none(cx, RENAME);
        let mut skip_serializing = BoolAttr::none(cx, SKIP_SERIALIZING);
        let mut skip_deserializing = BoolAttr::none(cx, SKIP_DESERIALIZING);
        let mut skip_serializing_if = Attr::none(cx, SKIP_SERIALIZING_IF);
        let mut default = Attr::none(cx, DEFAULT);
        let mut serialize_with = Attr::none(cx, SERIALIZE_WITH);
        let mut deserialize_with = Attr::none(cx, DESERIALIZE_WITH);
        let mut ser_bound = Attr::none(cx, BOUND);
        let mut de_bound = Attr::none(cx, BOUND);
        let mut borrowed_lifetimes = Attr::none(cx, BORROW);
        let mut getter = Attr::none(cx, GETTER);
        let mut flatten = BoolAttr::none(cx, FLATTEN);

        let ident = match &field.ident {
            Some(ident) => Name::from(&unraw(ident)),
            None => Name {
                value: index.to_string(),
                span: Span::call_site(),
            },
        };

        if let Some(borrow_attribute) = attrs.and_then(|variant| variant.borrow.as_ref()) {
            if let Ok(borrowable) = borrowable_lifetimes(cx, &ident, field) {
                if let Some(lifetimes) = &borrow_attribute.lifetimes {
                    for lifetime in lifetimes {
                        if !borrowable.contains(lifetime) {
                            let msg =
                                format!("field `{}` does not have lifetime {}", ident, lifetime);
                            cx.error_spanned_by(field, msg);
                        }
                    }
                    borrowed_lifetimes.set(&borrow_attribute.path, lifetimes.clone());
                } else {
                    borrowed_lifetimes.set(&borrow_attribute.path, borrowable);
                }
            }
        }

        for attr in &field.attrs {
            if attr.path() != SERDE {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == RENAME {
                    // #[serde(rename = "foo")]
                    // #[serde(rename(serialize = "foo", deserialize = "bar"))]
                    let (ser, de) = get_multiple_renames(cx, &meta)?;
                    ser_name.set_opt(&meta.path, ser.as_ref().map(Name::from));
                    for de_value in de {
                        de_name.set_if_none(Name::from(&de_value));
                        de_aliases.insert(&meta.path, Name::from(&de_value));
                    }
                } else if meta.path == ALIAS {
                    // #[serde(alias = "foo")]
                    if let Some(s) = get_lit_str(cx, ALIAS, &meta)? {
                        de_aliases.insert(&meta.path, Name::from(&s));
                    }
                } else if meta.path == DEFAULT {
                    if meta.input.peek(Token![=]) {
                        // #[serde(default = "...")]
                        if let Some(path) = parse_lit_into_expr_path(cx, DEFAULT, &meta)? {
                            default.set(&meta.path, Default::Path(path));
                        }
                    } else {
                        // #[serde(default)]
                        default.set(&meta.path, Default::Default);
                    }
                } else if meta.path == SKIP_SERIALIZING {
                    // #[serde(skip_serializing)]
                    skip_serializing.set_true(&meta.path);
                } else if meta.path == SKIP_DESERIALIZING {
                    // #[serde(skip_deserializing)]
                    skip_deserializing.set_true(&meta.path);
                } else if meta.path == SKIP {
                    // #[serde(skip)]
                    skip_serializing.set_true(&meta.path);
                    skip_deserializing.set_true(&meta.path);
                } else if meta.path == SKIP_SERIALIZING_IF {
                    // #[serde(skip_serializing_if = "...")]
                    if let Some(path) = parse_lit_into_expr_path(cx, SKIP_SERIALIZING_IF, &meta)? {
                        skip_serializing_if.set(&meta.path, path);
                    }
                } else if meta.path == SERIALIZE_WITH {
                    // #[serde(serialize_with = "...")]
                    if let Some(path) = parse_lit_into_expr_path(cx, SERIALIZE_WITH, &meta)? {
                        serialize_with.set(&meta.path, path);
                    }
                } else if meta.path == DESERIALIZE_WITH {
                    // #[serde(deserialize_with = "...")]
                    if let Some(path) = parse_lit_into_expr_path(cx, DESERIALIZE_WITH, &meta)? {
                        deserialize_with.set(&meta.path, path);
                    }
                } else if meta.path == WITH {
                    // #[serde(with = "...")]
                    if let Some(path) = parse_lit_into_expr_path(cx, WITH, &meta)? {
                        let mut ser_path = path.clone();
                        ser_path
                            .path
                            .segments
                            .push(Ident::new("serialize", ser_path.span()).into());
                        serialize_with.set(&meta.path, ser_path);
                        let mut de_path = path;
                        de_path
                            .path
                            .segments
                            .push(Ident::new("deserialize", de_path.span()).into());
                        deserialize_with.set(&meta.path, de_path);
                    }
                } else if meta.path == BOUND {
                    // #[serde(bound = "T: SomeBound")]
                    // #[serde(bound(serialize = "...", deserialize = "..."))]
                    let (ser, de) = get_where_predicates(cx, &meta)?;
                    ser_bound.set_opt(&meta.path, ser);
                    de_bound.set_opt(&meta.path, de);
                } else if meta.path == BORROW {
                    if meta.input.peek(Token![=]) {
                        // #[serde(borrow = "'a + 'b")]
                        let lifetimes = parse_lit_into_lifetimes(cx, &meta)?;
                        if let Ok(borrowable) = borrowable_lifetimes(cx, &ident, field) {
                            for lifetime in &lifetimes {
                                if !borrowable.contains(lifetime) {
                                    let msg = format!(
                                        "field `{}` does not have lifetime {}",
                                        ident, lifetime,
                                    );
                                    cx.error_spanned_by(field, msg);
                                }
                            }
                            borrowed_lifetimes.set(&meta.path, lifetimes);
                        }
                    } else {
                        // #[serde(borrow)]
                        if let Ok(borrowable) = borrowable_lifetimes(cx, &ident, field) {
                            borrowed_lifetimes.set(&meta.path, borrowable);
                        }
                    }
                } else if meta.path == GETTER {
                    // #[serde(getter = "...")]
                    if let Some(path) = parse_lit_into_expr_path(cx, GETTER, &meta)? {
                        getter.set(&meta.path, path);
                    }
                } else if meta.path == FLATTEN {
                    // #[serde(flatten)]
                    flatten.set_true(&meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown serde field attribute `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        // Is skip_deserializing, initialize the field to Default::default() unless a
        // different default is specified by `#[serde(default = "...")]` on
        // ourselves or our container (e.g. the struct we are in).
        if let Default::None = *container_default {
            if skip_deserializing.0.value.is_some() {
                default.set_if_none(Default::Default);
            }
        }

        let mut borrowed_lifetimes = borrowed_lifetimes.get().unwrap_or_default();
        if !borrowed_lifetimes.is_empty() {
            // Cow<str> and Cow<[u8]> never borrow by default:
            //
            //     impl<'de, 'a, T: ?Sized> Deserialize<'de> for Cow<'a, T>
            //
            // A #[serde(borrow)] attribute enables borrowing that corresponds
            // roughly to these impls:
            //
            //     impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, str>
            //     impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, [u8]>
            if is_cow(&field.ty, is_str) {
                let mut path = syn::Path {
                    leading_colon: None,
                    segments: Punctuated::new(),
                };
                let span = Span::call_site();
                path.segments.push(Ident::new("_serde", span).into());
                path.segments.push(Ident::new("__private", span).into());
                path.segments.push(Ident::new("de", span).into());
                path.segments
                    .push(Ident::new("borrow_cow_str", span).into());
                let expr = syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                };
                deserialize_with.set_if_none(expr);
            } else if is_cow(&field.ty, is_slice_u8) {
                let mut path = syn::Path {
                    leading_colon: None,
                    segments: Punctuated::new(),
                };
                let span = Span::call_site();
                path.segments.push(Ident::new("_serde", span).into());
                path.segments.push(Ident::new("__private", span).into());
                path.segments.push(Ident::new("de", span).into());
                path.segments
                    .push(Ident::new("borrow_cow_bytes", span).into());
                let expr = syn::ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path,
                };
                deserialize_with.set_if_none(expr);
            }
        } else if is_implicitly_borrowed(&field.ty) {
            // Types &str and &[u8] are always implicitly borrowed. No need for
            // a #[serde(borrow)].
            collect_lifetimes(&field.ty, &mut borrowed_lifetimes);
        }

        Field {
            name: MultiName::from_attrs(ident, ser_name, de_name, Some(de_aliases)),
            skip_serializing: skip_serializing.get(),
            skip_deserializing: skip_deserializing.get(),
            skip_serializing_if: skip_serializing_if.get(),
            default: default.get().unwrap_or(Default::None),
            serialize_with: serialize_with.get(),
            deserialize_with: deserialize_with.get(),
            ser_bound: ser_bound.get(),
            de_bound: de_bound.get(),
            borrowed_lifetimes,
            getter: getter.get(),
            flatten: flatten.get(),
            transparent: false,
        }
    }