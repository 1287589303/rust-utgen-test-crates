pub fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Self {
        let mut ser_name = Attr::none(cx, RENAME);
        let mut de_name = Attr::none(cx, RENAME);
        let mut transparent = BoolAttr::none(cx, TRANSPARENT);
        let mut deny_unknown_fields = BoolAttr::none(cx, DENY_UNKNOWN_FIELDS);
        let mut default = Attr::none(cx, DEFAULT);
        let mut rename_all_ser_rule = Attr::none(cx, RENAME_ALL);
        let mut rename_all_de_rule = Attr::none(cx, RENAME_ALL);
        let mut rename_all_fields_ser_rule = Attr::none(cx, RENAME_ALL_FIELDS);
        let mut rename_all_fields_de_rule = Attr::none(cx, RENAME_ALL_FIELDS);
        let mut ser_bound = Attr::none(cx, BOUND);
        let mut de_bound = Attr::none(cx, BOUND);
        let mut untagged = BoolAttr::none(cx, UNTAGGED);
        let mut internal_tag = Attr::none(cx, TAG);
        let mut content = Attr::none(cx, CONTENT);
        let mut type_from = Attr::none(cx, FROM);
        let mut type_try_from = Attr::none(cx, TRY_FROM);
        let mut type_into = Attr::none(cx, INTO);
        let mut remote = Attr::none(cx, REMOTE);
        let mut field_identifier = BoolAttr::none(cx, FIELD_IDENTIFIER);
        let mut variant_identifier = BoolAttr::none(cx, VARIANT_IDENTIFIER);
        let mut serde_path = Attr::none(cx, CRATE);
        let mut expecting = Attr::none(cx, EXPECTING);
        let mut non_exhaustive = false;

        for attr in &item.attrs {
            if attr.path() != SERDE {
                non_exhaustive |=
                    matches!(&attr.meta, syn::Meta::Path(path) if path == NON_EXHAUSTIVE);
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
                    let (ser, de) = get_renames(cx, RENAME, &meta)?;
                    ser_name.set_opt(&meta.path, ser.as_ref().map(Name::from));
                    de_name.set_opt(&meta.path, de.as_ref().map(Name::from));
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
                } else if meta.path == RENAME_ALL_FIELDS {
                    // #[serde(rename_all_fields = "foo")]
                    // #[serde(rename_all_fields(serialize = "foo", deserialize = "bar"))]
                    let one_name = meta.input.peek(Token![=]);
                    let (ser, de) = get_renames(cx, RENAME_ALL_FIELDS, &meta)?;

                    match item.data {
                        syn::Data::Enum(_) => {
                            if let Some(ser) = ser {
                                match RenameRule::from_str(&ser.value()) {
                                    Ok(rename_rule) => {
                                        rename_all_fields_ser_rule.set(&meta.path, rename_rule);
                                    }
                                    Err(err) => cx.error_spanned_by(ser, err),
                                }
                            }
                            if let Some(de) = de {
                                match RenameRule::from_str(&de.value()) {
                                    Ok(rename_rule) => {
                                        rename_all_fields_de_rule.set(&meta.path, rename_rule);
                                    }
                                    Err(err) => {
                                        if !one_name {
                                            cx.error_spanned_by(de, err);
                                        }
                                    }
                                }
                            }
                        }
                        syn::Data::Struct(_) => {
                            let msg = "#[serde(rename_all_fields)] can only be used on enums";
                            cx.syn_error(meta.error(msg));
                        }
                        syn::Data::Union(_) => {
                            let msg = "#[serde(rename_all_fields)] can only be used on enums";
                            cx.syn_error(meta.error(msg));
                        }
                    }
                } else if meta.path == TRANSPARENT {
                    // #[serde(transparent)]
                    transparent.set_true(meta.path);
                } else if meta.path == DENY_UNKNOWN_FIELDS {
                    // #[serde(deny_unknown_fields)]
                    deny_unknown_fields.set_true(meta.path);
                } else if meta.path == DEFAULT {
                    if meta.input.peek(Token![=]) {
                        // #[serde(default = "...")]
                        if let Some(path) = parse_lit_into_expr_path(cx, DEFAULT, &meta)? {
                            match &item.data {
                                syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
                                    syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                                        default.set(&meta.path, Default::Path(path));
                                    }
                                    syn::Fields::Unit => {
                                        let msg = "#[serde(default = \"...\")] can only be used on structs that have fields";
                                        cx.syn_error(meta.error(msg));
                                    }
                                },
                                syn::Data::Enum(_) => {
                                    let msg = "#[serde(default = \"...\")] can only be used on structs";
                                    cx.syn_error(meta.error(msg));
                                }
                                syn::Data::Union(_) => {
                                    let msg = "#[serde(default = \"...\")] can only be used on structs";
                                    cx.syn_error(meta.error(msg));
                                }
                            }
                        }
                    } else {
                        // #[serde(default)]
                        match &item.data {
                            syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
                                syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                                    default.set(meta.path, Default::Default);
                                }
                                syn::Fields::Unit => {
                                    let msg = "#[serde(default)] can only be used on structs that have fields";
                                    cx.error_spanned_by(fields, msg);
                                }
                            },
                            syn::Data::Enum(_) => {
                                let msg = "#[serde(default)] can only be used on structs";
                                cx.syn_error(meta.error(msg));
                            }
                            syn::Data::Union(_) => {
                                let msg = "#[serde(default)] can only be used on structs";
                                cx.syn_error(meta.error(msg));
                            }
                        }
                    }
                } else if meta.path == BOUND {
                    // #[serde(bound = "T: SomeBound")]
                    // #[serde(bound(serialize = "...", deserialize = "..."))]
                    let (ser, de) = get_where_predicates(cx, &meta)?;
                    ser_bound.set_opt(&meta.path, ser);
                    de_bound.set_opt(&meta.path, de);
                } else if meta.path == UNTAGGED {
                    // #[serde(untagged)]
                    match item.data {
                        syn::Data::Enum(_) => {
                            untagged.set_true(&meta.path);
                        }
                        syn::Data::Struct(_) => {
                            let msg = "#[serde(untagged)] can only be used on enums";
                            cx.syn_error(meta.error(msg));
                        }
                        syn::Data::Union(_) => {
                            let msg = "#[serde(untagged)] can only be used on enums";
                            cx.syn_error(meta.error(msg));
                        }
                    }
                } else if meta.path == TAG {
                    // #[serde(tag = "type")]
                    if let Some(s) = get_lit_str(cx, TAG, &meta)? {
                        match &item.data {
                            syn::Data::Enum(_) => {
                                internal_tag.set(&meta.path, s.value());
                            }
                            syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
                                syn::Fields::Named(_) => {
                                    internal_tag.set(&meta.path, s.value());
                                }
                                syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                                    let msg = "#[serde(tag = \"...\")] can only be used on enums and structs with named fields";
                                    cx.syn_error(meta.error(msg));
                                }
                            },
                            syn::Data::Union(_) => {
                                let msg = "#[serde(tag = \"...\")] can only be used on enums and structs with named fields";
                                cx.syn_error(meta.error(msg));
                            }
                        }
                    }
                } else if meta.path == CONTENT {
                    // #[serde(content = "c")]
                    if let Some(s) = get_lit_str(cx, CONTENT, &meta)? {
                        match &item.data {
                            syn::Data::Enum(_) => {
                                content.set(&meta.path, s.value());
                            }
                            syn::Data::Struct(_) => {
                                let msg = "#[serde(content = \"...\")] can only be used on enums";
                                cx.syn_error(meta.error(msg));
                            }
                            syn::Data::Union(_) => {
                                let msg = "#[serde(content = \"...\")] can only be used on enums";
                                cx.syn_error(meta.error(msg));
                            }
                        }
                    }
                } else if meta.path == FROM {
                    // #[serde(from = "Type")]
                    if let Some(from_ty) = parse_lit_into_ty(cx, FROM, &meta)? {
                        type_from.set_opt(&meta.path, Some(from_ty));
                    }
                } else if meta.path == TRY_FROM {
                    // #[serde(try_from = "Type")]
                    if let Some(try_from_ty) = parse_lit_into_ty(cx, TRY_FROM, &meta)? {
                        type_try_from.set_opt(&meta.path, Some(try_from_ty));
                    }
                } else if meta.path == INTO {
                    // #[serde(into = "Type")]
                    if let Some(into_ty) = parse_lit_into_ty(cx, INTO, &meta)? {
                        type_into.set_opt(&meta.path, Some(into_ty));
                    }
                } else if meta.path == REMOTE {
                    // #[serde(remote = "...")]
                    if let Some(path) = parse_lit_into_path(cx, REMOTE, &meta)? {
                        if is_primitive_path(&path, "Self") {
                            remote.set(&meta.path, item.ident.clone().into());
                        } else {
                            remote.set(&meta.path, path);
                        }
                    }
                } else if meta.path == FIELD_IDENTIFIER {
                    // #[serde(field_identifier)]
                    field_identifier.set_true(&meta.path);
                } else if meta.path == VARIANT_IDENTIFIER {
                    // #[serde(variant_identifier)]
                    variant_identifier.set_true(&meta.path);
                } else if meta.path == CRATE {
                    // #[serde(crate = "foo")]
                    if let Some(path) = parse_lit_into_path(cx, CRATE, &meta)? {
                        serde_path.set(&meta.path, path);
                    }
                } else if meta.path == EXPECTING {
                    // #[serde(expecting = "a message")]
                    if let Some(s) = get_lit_str(cx, EXPECTING, &meta)? {
                        expecting.set(&meta.path, s.value());
                    }
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown serde container attribute `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        let mut is_packed = false;
        for attr in &item.attrs {
            if attr.path() == REPR {
                let _ = attr.parse_args_with(|input: ParseStream| {
                    while let Some(token) = input.parse()? {
                        if let TokenTree::Ident(ident) = token {
                            is_packed |= ident == "packed";
                        }
                    }
                    Ok(())
                });
            }
        }

        Container {
            name: MultiName::from_attrs(Name::from(&unraw(&item.ident)), ser_name, de_name, None),
            transparent: transparent.get(),
            deny_unknown_fields: deny_unknown_fields.get(),
            default: default.get().unwrap_or(Default::None),
            rename_all_rules: RenameAllRules {
                serialize: rename_all_ser_rule.get().unwrap_or(RenameRule::None),
                deserialize: rename_all_de_rule.get().unwrap_or(RenameRule::None),
            },
            rename_all_fields_rules: RenameAllRules {
                serialize: rename_all_fields_ser_rule.get().unwrap_or(RenameRule::None),
                deserialize: rename_all_fields_de_rule.get().unwrap_or(RenameRule::None),
            },
            ser_bound: ser_bound.get(),
            de_bound: de_bound.get(),
            tag: decide_tag(cx, item, untagged, internal_tag, content),
            type_from: type_from.get(),
            type_try_from: type_try_from.get(),
            type_into: type_into.get(),
            remote: remote.get(),
            identifier: decide_identifier(cx, item, field_identifier, variant_identifier),
            serde_path: serde_path.get(),
            is_packed,
            expecting: expecting.get(),
            non_exhaustive,
        }
    }