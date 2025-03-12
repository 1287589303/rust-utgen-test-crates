pub fn from_ast(cx: &Ctxt, variant: &syn::Variant) -> Self {
        let mut ser_name = Attr::none(cx, RENAME);
        let mut de_name = Attr::none(cx, RENAME);
        let mut de_aliases = VecAttr::none(cx, RENAME);
        let mut skip_deserializing = BoolAttr::none(cx, SKIP_DESERIALIZING);
        let mut skip_serializing = BoolAttr::none(cx, SKIP_SERIALIZING);
        let mut rename_all_ser_rule = Attr::none(cx, RENAME_ALL);
        let mut rename_all_de_rule = Attr::none(cx, RENAME_ALL);
        let mut ser_bound = Attr::none(cx, BOUND);
        let mut de_bound = Attr::none(cx, BOUND);
        let mut other = BoolAttr::none(cx, OTHER);
        let mut serialize_with = Attr::none(cx, SERIALIZE_WITH);
        let mut deserialize_with = Attr::none(cx, DESERIALIZE_WITH);
        let mut borrow = Attr::none(cx, BORROW);
        let mut untagged = BoolAttr::none(cx, UNTAGGED);

        for attr in &variant.attrs {
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
                } else if meta.path == RENAME_ALL {
                    // #[serde(rename_all = "foo")]
                    // #[serde(rename_all(serialize = "foo", deserialize = "bar"))]
                    let one_name = meta.input.peek(Token![=]);
                    let (ser, de) = get_renames(cx, RENAME_ALL, &meta)?;
                    if let Some(ser) = ser {
                        match RenameRule::from_str(&ser.value()) {
                            Ok(rename_rule) => rename_all_ser_rule.set(&meta.path, rename_rule),
                            Err(err) => cx.error_spanned_by(ser, err),
                        }
                    }
                    if let Some(de) = de {
                        match RenameRule::from_str(&de.value()) {
                            Ok(rename_rule) => rename_all_de_rule.set(&meta.path, rename_rule),
                            Err(err) => {
                                if !one_name {
                                    cx.error_spanned_by(de, err);
                                }
                            }
                        }
                    }
                } else if meta.path == SKIP {
                    // #[serde(skip)]
                    skip_serializing.set_true(&meta.path);
                    skip_deserializing.set_true(&meta.path);
                } else if meta.path == SKIP_DESERIALIZING {
                    // #[serde(skip_deserializing)]
                    skip_deserializing.set_true(&meta.path);
                } else if meta.path == SKIP_SERIALIZING {
                    // #[serde(skip_serializing)]
                    skip_serializing.set_true(&meta.path);
                } else if meta.path == OTHER {
                    // #[serde(other)]
                    other.set_true(&meta.path);
                } else if meta.path == BOUND {
                    // #[serde(bound = "T: SomeBound")]
                    // #[serde(bound(serialize = "...", deserialize = "..."))]
                    let (ser, de) = get_where_predicates(cx, &meta)?;
                    ser_bound.set_opt(&meta.path, ser);
                    de_bound.set_opt(&meta.path, de);
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
                } else if meta.path == BORROW {
                    let borrow_attribute = if meta.input.peek(Token![=]) {
                        // #[serde(borrow = "'a + 'b")]
                        let lifetimes = parse_lit_into_lifetimes(cx, &meta)?;
                        BorrowAttribute {
                            path: meta.path.clone(),
                            lifetimes: Some(lifetimes),
                        }
                    } else {
                        // #[serde(borrow)]
                        BorrowAttribute {
                            path: meta.path.clone(),
                            lifetimes: None,
                        }
                    };
                    match &variant.fields {
                        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                            borrow.set(&meta.path, borrow_attribute);
                        }
                        _ => {
                            let msg = "#[serde(borrow)] may only be used on newtype variants";
                            cx.error_spanned_by(variant, msg);
                        }
                    }
                } else if meta.path == UNTAGGED {
                    untagged.set_true(&meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown serde variant attribute `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        Variant {
            name: MultiName::from_attrs(
                Name::from(&unraw(&variant.ident)),
                ser_name,
                de_name,
                Some(de_aliases),
            ),
            rename_all_rules: RenameAllRules {
                serialize: rename_all_ser_rule.get().unwrap_or(RenameRule::None),
                deserialize: rename_all_de_rule.get().unwrap_or(RenameRule::None),
            },
            ser_bound: ser_bound.get(),
            de_bound: de_bound.get(),
            skip_deserializing: skip_deserializing.get(),
            skip_serializing: skip_serializing.get(),
            other: other.get(),
            serialize_with: serialize_with.get(),
            deserialize_with: deserialize_with.get(),
            borrow: borrow.get(),
            untagged: untagged.get(),
        }
    }